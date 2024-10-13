use log::info;

pub mod hello_command;

pub fn reg_commands(_app: &tauri::App) {
    info!("register commands...");
    hello_command::reg_commands();
    info!("register commands finish!");
}
