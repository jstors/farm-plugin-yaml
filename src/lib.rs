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
use serde_yaml::Value;

#[farm_plugin]
pub struct FarmPluginYaml {}

impl FarmPluginYaml {
  fn new(config: &Config, options: String) -> Self {
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
      let value: Value = Value::String(param.content.to_owned());
      let yaml_str: String = serde_yaml::from_value(value).expect("Failed to parse YAML");
      // Split the string into key-value pairs.
      let key_values: Vec<&str> = yaml_str.split("\n").collect();
      let mut data = HashMap::new();

      for pair in key_values {
        let parts: Vec<&str> = pair.split(":").collect();
        if parts.len() == 2 {
          let key: &str = parts[0].trim();
          let value = parts[1].trim();
          data.insert(key.to_string(), value.to_string());
        }
      }

      let json_data = json!(data);
      let result_code = format!("module.exports = {}", json_data);

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
