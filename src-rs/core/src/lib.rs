use anyhow::Result;

pub mod api;

use log::info;

pub use lazynote_common::*;

pub fn init(app: &mut tauri::App) -> Result<()> {
    info!("init...");
    lazynote_libs::init_plugins(app)?;
    api::reg_commands(app);
    info!("init finish!");
    Ok(())
}
