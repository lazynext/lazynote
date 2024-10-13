use anyhow::{anyhow, Result};

use serde::Serialize;
use serde_json::Value;

pub enum CommandResValue {
    Json(Value),
    Raw(Vec<u8>),
}

pub trait CommandResultTrait {
    fn json<T>(data: T) -> Self
    where
        T: Serialize;
    fn raw(data: Vec<u8>) -> Self;
}

pub type CommandResult = Result<Option<CommandResValue>>;

impl CommandResValue {
    pub fn json<T>(data: T) -> CommandResult
    where
        T: Serialize,
    {
        match serde_json::to_value(data) {
            Ok(r) => Ok(Some(CommandResValue::Json(r))),
            Err(err) => Err(anyhow!(err)),
        }
    }
    pub fn raw(data: Vec<u8>) -> CommandResult {
        Ok(Some(CommandResValue::Raw(data)))
    }
    pub fn ok() -> CommandResult {
        Ok(None)
    }
    pub fn err(err: String) -> CommandResult {
        Err(anyhow!(err))
    }
}
