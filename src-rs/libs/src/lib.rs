use anyhow::Result;
use log::info;

pub mod os;
pub mod path;
pub mod store;

pub fn init_plugins(app: &tauri::App) -> Result<()> {
    info!("init plugins...");
    let handle = app.handle();
    handle.plugin(os::init_plugin()?)?;
    handle.plugin(path::init_plugin(app)?)?;
    handle.plugin(store::init_plugin()?)?;
    info!("init plugins finish!");
    Ok(())
}
