use anyhow::{anyhow, Result};
use std::collections::HashMap;

use crate::core;
use lazynote_core::{
    commands::{
        context::{CommandArgsValue, CommandContext},
        result::{CommandResValue, CommandResult},
    },
    state::AppState,
};
use tauri::ipc::{InvokeBody, Request, Response};

fn parse_invoke_args<'a>(
    request: &'a Request<'a>,
    state: tauri::State<AppState>,
    app_handle: tauri::AppHandle,
    webview_window: tauri::WebviewWindow,
) -> Result<(String, CommandContext)> {
    let headers = request
        .headers()
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_str().unwrap_or("").to_string()))
        .collect::<HashMap<String, String>>();
    let key = headers.get("key");
    match key {
        Some(k) => match headers.get("raw") {
            Some(_) => {
                let data = if let InvokeBody::Raw(data) = request.body() {
                    Some(CommandArgsValue::Raw(data))
                } else {
                    None
                };
                Ok((
                    k.clone(),
                    CommandContext::new(headers, data, state.inner(), app_handle, webview_window),
                ))
            }
            None => {
                let data = if let InvokeBody::Json(data) = request.body() {
                    match data.get("args") {
                        Some(args) => Some(CommandArgsValue::Json(args)),
                        None => None,
                    }
                } else {
                    None
                };
                Ok((
                    k.clone(),
                    CommandContext::new(headers, data, state.inner(), app_handle, webview_window),
                ))
            }
        },
        None => Err(anyhow!("error invoke, notfound key")),
    }
}

pub fn parse_to_invoke_result(res: CommandResult) -> Result<Response> {
    let r = match res {
        Ok(r) => match r {
            Some(command_res) => match command_res {
                CommandResValue::Json(json) => Ok(Some(InvokeBody::Json(json))),
                CommandResValue::Raw(raw) => Ok(Some(InvokeBody::Raw(raw))),
            },
            None => Ok(None),
        },
        Err(err) => Err(err),
    };
    match r {
        Ok(res) => match res {
            Some(v) => Ok(Response::new(v)),
            None => Ok(Response::new(InvokeBody::default())),
        },
        Err(err) => Err(err),
    }
}

#[tauri::command]
pub fn invoke(
    request: Request<'_>,
    state: tauri::State<AppState>,
    app_handle: tauri::AppHandle,
    webview_window: tauri::WebviewWindow,
) -> Result<Response, String> {
    match parse_invoke_args(&request, state, app_handle, webview_window) {
        Ok((k, ctx)) => {
            let res = core::core_invoke_command(&k, ctx);
            parse_to_invoke_result(res).map_err(|e| e.to_string())
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn invoke_async<'a>(
    request: Request<'a>,
    state: tauri::State<'a, AppState>,
    app_handle: tauri::AppHandle,
    webview_window: tauri::WebviewWindow,
) -> Result<Response, String> {
    match parse_invoke_args(&request, state, app_handle, webview_window) {
        Ok((k, ctx)) => {
            let res = core::core_invoke_command_async(&k, ctx).await;
            parse_to_invoke_result(res).map_err(|e| e.to_string())
        }
        Err(err) => Err(err.to_string()),
    }
}
