use farm_plugin_yaml::FarmPluginYaml;
use farmfe_core::module::ModuleType;
use farmfe_core::plugin::{Plugin, PluginTransformHookParam};
use farmfe_core::{config::Config, context::CompilationContext};
use farmfe_testing_helpers::fixture;
use farmfe_toolkit::fs::read_file_utf8;
use std::fs;
use std::{collections::HashMap, sync::Arc};

#[test]
fn test() {
  fixture!("tests/fixtures/r.yml", |file, _cwd| {
    let resolve_path = file.to_string_lossy().to_string();

    let config = Config {
      input: HashMap::from([("index".to_string(), resolve_path.clone())]),
      ..Default::default()
    };

    let plugin_yaml = Arc::new(FarmPluginYaml::new(&config, "".to_string()));

    let context = CompilationContext::new(config, vec![plugin_yaml.clone()]).unwrap();

    let content = read_file_utf8(&resolve_path).unwrap();

    let transformed = plugin_yaml
      .transform(
        &PluginTransformHookParam {
          resolved_path: &resolve_path,
          content,
          module_type: ModuleType::Custom("yaml".to_string()),
          query: vec![],
          meta: HashMap::from([]),
          module_id: resolve_path.clone(),
          source_map_chain: vec![],
        },
        &Arc::new(context),
      )
      .unwrap()
      .unwrap();
    let dir = file.parent().unwrap();

    fs::write(dir.join("output.js"), transformed.content.clone()).unwrap();
    let expected = std::fs::read_to_string(dir.join("output.js")).unwrap();

    assert_eq!(expected, transformed.content)
  });
}
