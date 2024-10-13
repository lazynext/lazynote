use anyhow::Result;
use lazynote_core::{
    commands::{
        context::CommandContext, invoke_command, invoke_command_async, result::CommandResult,
    },
    state::AppState,
};

pub fn init(app: &mut tauri::App) -> Result<()> {
    lazynote_core::init(app)
}

pub fn core_invoke_command(key: &str, ctx: CommandContext) -> CommandResult {
    invoke_command(key, ctx)
}

pub async fn core_invoke_command_async(key: &str, ctx: CommandContext) -> CommandResult {
    invoke_command_async(key, ctx).await
}

pub fn create_state() -> AppState {
    AppState::new()
}
