use anyhow::anyhow;
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};

use super::super::{context::CommandContext, handler::CommandHandlerAsync, result::CommandResult};

use tokio::sync::RwLock;

type CommandHandlerResultAsync<'a> = Pin<Box<dyn Future<Output = CommandResult> + Send + 'a>>;

type CommandHandlerValueAsync =
    Box<dyn for<'b> Fn(&'b str, CommandContext) -> CommandHandlerResultAsync + Send + Sync>;

struct CommandsAsync {
    commands: HashMap<&'static str, CommandHandlerValueAsync>,
}

impl CommandsAsync {
    fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    fn reg_command<T, H>(&mut self, command: &'static str, f: H)
    where
        H: CommandHandlerAsync<T> + Send + Sync + 'static,
    {
        let f = std::sync::Arc::new(f);
        let wrapped_fn =
            for<'b> move |key: &'b str,
                          ctx: CommandContext|
                          -> Pin<Box<dyn Future<Output = CommandResult> + Send + 'b>> {
                let f = std::sync::Arc::clone(&f);
                Box::pin(async move {
                    let f = f.call(key, ctx);
                    f.await
                })
            };
        self.commands.insert(command, Box::new(wrapped_fn));
    }

    pub fn get_command(&self, key: &str) -> Option<&CommandHandlerValueAsync> {
        self.commands.get(key)
    }
}

lazy_static::lazy_static! {
    static ref COMMANDS_ASYNC: Arc<RwLock<CommandsAsync>> = Arc::new(RwLock::new(CommandsAsync::new()));
}

pub fn reg_command_async<T, H>(command: &'static str, handler: H)
where
    H: CommandHandlerAsync<T> + Send + Sync + 'static,
{
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let mut commands = COMMANDS_ASYNC.write().await;
        commands.reg_command(command, handler);
    });
    // COMMANDS_ASYNC
    //     .write()
    //     .unwrap()
    //     .reg_command(command, handler);
}

pub async fn invoke_command_async(key: &str, ctx: CommandContext) -> CommandResult {
    let commands = COMMANDS_ASYNC.read().await;
    // let commands = COMMANDS_ASYNC.read().unwrap();
    if let Some(cmd) = commands.get_command(key) {
        cmd(key, ctx).await
    } else {
        Err(anyhow!("async command not found: {}", key))
    }
}
