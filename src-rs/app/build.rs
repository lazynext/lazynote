use std::{collections::HashMap, env, fs};

use serde::{Deserialize, Serialize};
use tauri_build::{try_build, Attributes, DefaultPermissionRule, InlinedPlugin};

#[derive(Default, Serialize, Deserialize)]
struct PluginDefine {
    name: String,
    commands: Vec<String>,
}

#[derive(Default, Serialize, Deserialize)]
struct LibConfig {
    #[serde(rename = "plugins")]
    plugins: Vec<PluginDefine>,
}

lazy_static::lazy_static! {
    static ref LIB_CONFIG: LibConfig = {
        let libs_path = env::current_dir().unwrap().join("../libs/libs.json");
        serde_json::from_str(
            fs::read_to_string(&libs_path)
                .expect(format!("notfound file: {}", libs_path.display()).as_str())
                .as_str(),
        )
        .expect("parse json error")
    };
    static ref LIB_CONFIG_COMMANDS: HashMap<String, Vec<&'static str>> = {
        let mut map = HashMap::new();
        for p in LIB_CONFIG.plugins.iter() {
            map.insert(p.name.clone(), p.commands.iter().map(|c| string_to_static_str(c.clone()))
            .collect::<Vec<&'static str>>());
        }
        map
    };
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn main() {
    let mut attributes = Attributes::new();
    for (name, commands) in LIB_CONFIG_COMMANDS.iter() {
        attributes = attributes.plugin(
            name.as_str(),
            InlinedPlugin::new()
                .commands(&commands)
                .default_permission(DefaultPermissionRule::AllowAllCommands),
        );
    }
    try_build(attributes).expect("failed to run tauri-build");
}
