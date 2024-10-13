use std::collections::HashMap;

use serde_json::Value;

use crate::state::AppState;

#[derive(Clone, Debug)]
pub struct CommandContext {
    headers: HashMap<String, String>,
    args: Option<CommandArgsValue>,
    state: *const AppState,
    app_handle: tauri::AppHandle,
    webview_window: tauri::WebviewWindow,
}

unsafe impl Send for CommandContext {}

impl CommandContext {
    pub fn new(
        headers: HashMap<String, String>,
        args: Option<CommandArgsValue>,
        state: &AppState,
        app_handle: tauri::AppHandle,
        webview_window: tauri::WebviewWindow,
    ) -> Self {
        Self {
            headers,
            args,
            state,
            app_handle,
            webview_window,
        }
    }
    pub fn headers(&self) -> *const HashMap<String, String> {
        &self.headers
    }
    pub fn args(&self) -> &Option<CommandArgsValue> {
        &self.args
    }
    pub fn state(&self) -> *const AppState {
        self.state
    }
    pub fn app_handle(&self) -> *const tauri::AppHandle {
        &self.app_handle
    }
    pub fn webview_window(&self) -> *const tauri::WebviewWindow {
        &self.webview_window
    }
}

#[derive(Clone, Debug)]
pub enum CommandArgsValue {
    Json(*const Value),
    Raw(*const Vec<u8>),
}
