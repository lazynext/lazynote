use anyhow::{anyhow, Result};

use super::super::{
    body::{AppHandleBody, AppStateBody, HeadersBody, JsonBody, RawBody, WebViewWindowBody},
    context::{CommandArgsValue, CommandContext},
};

pub trait FromCommandArgs: Sized {
    fn from_args(key: &str, ctx: &CommandContext) -> Result<Self>;
}

impl<T> FromCommandArgs for JsonBody<T>
where
    T: for<'de> serde::Deserialize<'de>,
{
    fn from_args(key: &str, ctx: &CommandContext) -> Result<Self> {
        match ctx.args() {
            Some(CommandArgsValue::Json(json)) => {
                let input_t = T::deserialize(unsafe { &**json })?;
                Ok(JsonBody(input_t))
            }
            Some(CommandArgsValue::Raw(_)) => {
                Err(anyhow!("expected [JSON] body but got raw body: {}", key))
            }
            None => Err(anyhow!(
                "expected [JSON] body but no body provided: {}",
                key
            )),
        }
    }
}

impl FromCommandArgs for RawBody<'_> {
    fn from_args(key: &str, ctx: &CommandContext) -> Result<Self> {
        match ctx.args() {
            Some(CommandArgsValue::Raw(bytes)) => Ok(RawBody(unsafe { &**bytes })),
            Some(CommandArgsValue::Json(_)) => {
                Err(anyhow!("expected [RAW] body but got JSON body: {}", key))
            }
            None => Err(anyhow!("expected [RAW] body but no body provided: {}", key)),
        }
    }
}

impl FromCommandArgs for HeadersBody<'_> {
    fn from_args(_key: &str, ctx: &CommandContext) -> Result<Self> {
        Ok(HeadersBody(unsafe { &*ctx.headers() }))
    }
}

impl FromCommandArgs for AppStateBody<'_> {
    fn from_args(_key: &str, ctx: &CommandContext) -> Result<Self> {
        Ok(AppStateBody(unsafe { &*ctx.state() }))
    }
}

impl FromCommandArgs for AppHandleBody<'_> {
    fn from_args(_key: &str, ctx: &CommandContext) -> Result<Self> {
        Ok(AppHandleBody(unsafe { &*ctx.app_handle() }))
    }
}

impl FromCommandArgs for WebViewWindowBody<'_> {
    fn from_args(_key: &str, ctx: &CommandContext) -> Result<Self> {
        Ok(WebViewWindowBody(unsafe { &*ctx.webview_window() }))
    }
}

impl FromCommandArgs for () {
    fn from_args(_: &str, _: &CommandContext) -> Result<Self> {
        Ok(())
    }
}
