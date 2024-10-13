use anyhow::Result;
use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    Runtime,
};

use super::os;

#[command]
fn arch() -> &'static str {
    os::arch()
}
#[command]
fn family() -> &'static str {
    os::family()
}
#[command]
fn hostname() -> String {
    os::hostname()
}
#[command]
fn locale() -> Option<String> {
    os::locale()
}
#[command]
fn os_type() -> &'static str {
    os::os_type()
}
#[command]
fn platform() -> &'static str {
    os::platform()
}
#[command]
fn version() -> Option<String> {
    os::version()
}

pub(crate) fn init_plugin<R: Runtime>() -> Result<TauriPlugin<R>> {
    Ok(Builder::new("os")
        .invoke_handler(tauri::generate_handler![
            arch, family, hostname, locale, os_type, platform, version
        ])
        .build())
}
