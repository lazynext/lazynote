mod commands;
mod state;

use state::create_state;
use tauri::{WebviewUrl, WebviewWindowBuilder};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let ctx = tauri::generate_context!("Tauri.toml");
    let builder = tauri::Builder::default();
    let third_log_filter = if cfg!(debug_assertions) {
        log::LevelFilter::Info
    } else {
        log::LevelFilter::Error
    };
    #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos",))]
    let builder = { builder.plugin(tauri_plugin_decorum::init()) };
    let builder = { commands::reg_commands(builder) };
    builder
        .manage(create_state())
        .plugin(
            tauri_plugin_log::Builder::new()
                .max_file_size(50_000 /* bytes */)
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("logs".to_string()),
                    },
                ))
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Webview,
                ))
                .level(log::LevelFilter::Info)
                .level_for("lazynote", log::LevelFilter::Info)
                .level_for("lazynote_core", log::LevelFilter::Info)
                .level_for("lazynote_libs", log::LevelFilter::Info)
                .level_for("lazynote_common", log::LevelFilter::Info)
                .level_for("tao", third_log_filter)
                .level_for(
                    "tao::platform_impl::platform::event_loop::runner",
                    log::LevelFilter::Error,
                )
                .build(),
        )
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        // .plugin(tauri_plugin_shell::init())
        // .plugin(tauri_plugin_updater::Builder::new().build())
        // .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let conf = app.config();
            let title = conf.product_name.as_ref().unwrap().as_str();
            let win_builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());

            #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos",))]
            let win_builder = win_builder
                .visible(false)
                .title(title)
                .inner_size(1024.0, 576.0);

            #[cfg(any(target_os = "windows"))]
            let win_builder = win_builder.decorations(false);

            let win = win_builder.build().unwrap();

            #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos",))]
            {
                use tauri_plugin_decorum::WebviewWindowExt;
                // 覆盖默认标题栏
                win.create_overlay_titlebar().unwrap();
                // 开发模式下自动打开devtools
                #[cfg(debug_assertions)]
                win.open_devtools();
            }

            #[cfg(target_os = "macos")]
            {
                // MacOS标题栏
                win.set_title_bar_style(tauri::TitleBarStyle::Overlay)
                    .unwrap();
                // 设置标题栏位置
                win.set_traffic_lights_inset(12.0, 16.0).unwrap();
                // // 设置窗体透明, 使用privateApi
                // win.make_transparent().unwrap();
                // // 设置窗体置顶
                // // NSWindowLevel: https://developer.apple.com/documentation/appkit/nswindowlevel
                // win.set_window_level(25).unwrap();
            }

            lazynote_plugins::init_plugins(app)?;

            Ok(())
        })
        .run(ctx)
        .expect("error while running tauri application");
}
