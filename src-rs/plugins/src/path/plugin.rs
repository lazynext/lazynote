use anyhow::Result;
use tauri::{
    command,
    plugin::{Builder, TauriPlugin},
    Runtime,
};

use super::path::{init_paths, paths};

#[command]
fn app_cache_dir() -> String {
    paths().app_cache_dir().to_string_lossy().into_owned()
}
#[command]
fn app_config_dir() -> String {
    paths().app_config_dir().to_string_lossy().into_owned()
}
#[command]
fn app_data_dir() -> String {
    paths().app_data_dir().to_string_lossy().into_owned()
}
#[command]
fn app_local_data_dir() -> String {
    paths().app_local_data_dir().to_string_lossy().into_owned()
}
#[command]
fn app_log_dir() -> String {
    paths().app_log_dir().to_string_lossy().into_owned()
}
#[command]
fn audio_dir() -> String {
    paths().audio_dir().to_string_lossy().into_owned()
}
#[command]
fn cache_dir() -> String {
    paths().cache_dir().to_string_lossy().into_owned()
}
#[command]
fn config_dir() -> String {
    paths().config_dir().to_string_lossy().into_owned()
}
#[command]
fn data_dir() -> String {
    paths().data_dir().to_string_lossy().into_owned()
}
#[command]
fn desktop_dir() -> String {
    paths().desktop_dir().to_string_lossy().into_owned()
}
#[command]
fn document_dir() -> String {
    paths().document_dir().to_string_lossy().into_owned()
}
#[command]
fn download_dir() -> String {
    paths().download_dir().to_string_lossy().into_owned()
}
#[command]
fn executable_dir() -> String {
    paths().executable_dir().to_string_lossy().into_owned()
}
#[command]
fn font_dir() -> String {
    paths().font_dir().to_string_lossy().into_owned()
}
#[command]
fn home_dir() -> String {
    paths().home_dir().to_string_lossy().into_owned()
}
#[command]
fn local_data_dir() -> String {
    paths().local_data_dir().to_string_lossy().into_owned()
}
#[command]
fn picture_dir() -> String {
    paths().picture_dir().to_string_lossy().into_owned()
}
#[command]
fn public_dir() -> String {
    paths().public_dir().to_string_lossy().into_owned()
}
#[command]
fn resource_dir() -> String {
    paths().resource_dir().to_string_lossy().into_owned()
}
#[command]
fn runtime_dir() -> String {
    paths().runtime_dir().to_string_lossy().into_owned()
}
#[command]
fn template_dir() -> String {
    paths().template_dir().to_string_lossy().into_owned()
}
#[command]
fn data_path() -> String {
    paths().data_path().to_string_lossy().into_owned()
}
#[command]
fn local_res_path() -> String {
    paths().local_res_path().to_string_lossy().into_owned()
}
#[command]
fn local_data_path() -> String {
    paths().local_data_path().to_string_lossy().into_owned()
}

pub(crate) fn init_plugin<R: Runtime>(app: &tauri::App) -> Result<TauriPlugin<R>> {
    init_paths(app)?;
    Ok(Builder::new("path")
        .invoke_handler(tauri::generate_handler![
            app_cache_dir,
            app_config_dir,
            app_data_dir,
            app_local_data_dir,
            app_log_dir,
            audio_dir,
            cache_dir,
            config_dir,
            data_dir,
            desktop_dir,
            document_dir,
            download_dir,
            executable_dir,
            font_dir,
            home_dir,
            local_data_dir,
            picture_dir,
            public_dir,
            resource_dir,
            runtime_dir,
            template_dir,
            data_path,
            local_res_path,
            local_data_path,
        ])
        .build())
}
