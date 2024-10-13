use anyhow::anyhow;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use super::super::{context::CommandContext, handler::CommandHandler, result::CommandResult};

type CommandHandlerValue = Box<dyn Fn(&str, CommandContext) -> CommandResult + Send + Sync>;

struct Commands {
    commands: HashMap<&'static str, CommandHandlerValue>,
}
impl Commands {
    fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }
    fn reg_command<T, H>(&mut self, command: &'static str, f: H)
    where
        H: CommandHandler<T>,
    {
        let wrapped_fn =
            move |key: &str, ctx: CommandContext| -> CommandResult { f.call(key, ctx) };
        self.commands.insert(command, Box::new(wrapped_fn));
    }
    pub fn get_command(&self, key: &str) -> Option<&CommandHandlerValue> {
        return self.commands.get(key);
    }
}

lazy_static::lazy_static! {
    static ref COMMANDS: Arc<RwLock<Commands>> = Arc::new(RwLock::new(Commands::new()));
}

pub fn reg_command<T, H>(command: &'static str, handler: H)
where
    H: CommandHandler<T>,
{
    COMMANDS.write().unwrap().reg_command(command, handler);
}

pub fn invoke_command(key: &str, ctx: CommandContext) -> CommandResult {
    let commands = COMMANDS.read().unwrap();
    if let Some(cmd) = commands.get_command(key) {
        cmd(key, ctx)
    } else {
        Err(anyhow!("async command not found: {}", key))
    }
}
