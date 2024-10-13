use std::collections::HashMap;

use crate::state::AppState;

pub trait BodyInner<T> {
    fn inner(&self) -> &T;
}

pub struct JsonBody<T>(pub T)
where
    T: for<'de> serde::Deserialize<'de>;
impl<T> BodyInner<T> for JsonBody<T>
where
    T: for<'de> serde::Deserialize<'de>,
{
    fn inner(&self) -> &T {
        &self.0
    }
}

pub struct RawBody<'a>(pub &'a Vec<u8>);
impl<'a> BodyInner<Vec<u8>> for RawBody<'a> {
    fn inner(&self) -> &Vec<u8> {
        &self.0
    }
}

pub struct HeadersBody<'a>(pub &'a HashMap<String, String>);
impl<'a> BodyInner<HashMap<String, String>> for HeadersBody<'a> {
    fn inner(&self) -> &HashMap<String, String> {
        &self.0
    }
}

pub struct AppStateBody<'a>(pub &'a AppState);
impl<'a> BodyInner<AppState> for AppStateBody<'a> {
    fn inner(&self) -> &AppState {
        &self.0
    }
}

pub struct AppHandleBody<'a>(pub &'a tauri::AppHandle);
impl<'a> BodyInner<tauri::AppHandle> for AppHandleBody<'a> {
    fn inner(&self) -> &tauri::AppHandle {
        &self.0
    }
}

pub struct WebViewWindowBody<'a>(pub &'a tauri::WebviewWindow);
impl<'a> BodyInner<tauri::WebviewWindow> for WebViewWindowBody<'a> {
    fn inner(&self) -> &tauri::WebviewWindow {
        &self.0
    }
}

pub struct NoArgsBody;
