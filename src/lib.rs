#![deny(clippy::all)]
use std::collections::HashMap;

use farmfe_core::{
  config::Config,
  module::ModuleType,
  plugin::{Plugin, PluginLoadHookResult, PluginTransformHookResult},
  serde_json::{self, json},
};
use farmfe_macro_plugin::farm_plugin;
use farmfe_toolkit::fs;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[farm_plugin]
pub struct FarmPluginYaml {}

#[derive(Debug, Deserialize, Serialize)]
enum ResultValue {
  List(Vec<Value>),
  Obj(HashMap<Value, Value>),
  Str(String),
  Value(serde_yaml::Value),
}

impl FarmPluginYaml {
  pub fn new(_config: &Config, _options: String) -> Self {
    Self {}
  }
}

impl Plugin for FarmPluginYaml {
  fn name(&self) -> &str {
    "FarmPluginYaml"
  }
  fn load(
    &self,
    param: &farmfe_core::plugin::PluginLoadHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
    _hook_context: &farmfe_core::plugin::PluginHookContext,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginLoadHookResult>> {
    if param.resolved_path.ends_with(".yaml") || param.resolved_path.ends_with(".yml") {
      return Ok(Some(PluginLoadHookResult {
        content: fs::read_file_utf8(param.resolved_path)?,
        module_type: ModuleType::Custom("yaml".to_string()),
        source_map: None,
      }));
    }
    Ok(None)
  }
  fn transform(
    &self,
    param: &farmfe_core::plugin::PluginTransformHookParam,
    _context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginTransformHookResult>> {
    if param.module_type == ModuleType::Custom(("yaml").to_string()) {
      let value = serde_yaml::from_str(&param.content).expect("Failed parse YAML");
      let mut result: HashMap<String, serde_json::Value> = HashMap::new();

      if let Value::Mapping(map) = value {
        for (k, v) in map {
          match v {
            Value::Sequence(seq) => {
              result.insert(
                key_to_string(&k),
                serde_json::Value::Array(
                  serde_json::to_value(seq)
                    .unwrap()
                    .as_array()
                    .unwrap()
                    .to_owned(),
                ),
              );
            }
            Value::Mapping(inner_map) => {
              let mut inner_obj = serde_json::Map::new();

              for (k, v) in inner_map {
                inner_obj.insert(
                  key_to_string(&k),
                  serde_json::to_value(trim_str(serde_yaml::to_string(&v).unwrap())).unwrap(),
                );
              }
              result.insert(key_to_string(&k), serde_json::Value::Object(inner_obj));
            }
            _ => {
              result.insert(
                key_to_string(&k),
                serde_json::Value::String(trim_str(serde_yaml::to_string(&v).unwrap())),
              );
            }
          }
        }
      };

      let result_code = format!("module.exports = {}", json!(&result));

      return Ok(Some(PluginTransformHookResult {
        content: result_code,
        module_type: Some(ModuleType::Js),
        source_map: None,
        ignore_previous_source_map: false,
      }));
    }

    Ok(None)
  }
}

fn key_to_string(key: &Value) -> String {
  serde_yaml::to_string(key).unwrap().trim().to_string()
}

fn trim_str(str: String) -> String {
  str.trim().to_string()
}
