use std::{
    path::PathBuf,
    sync::{Arc, LazyLock, RwLock, RwLockReadGuard},
};

use anyhow::{anyhow, Result};
use tauri::{AppHandle, Manager};

#[derive(Default)]
pub struct AppPaths {
    app_cache_dir: PathBuf,
    app_config_dir: PathBuf,
    app_data_dir: PathBuf,
    app_local_data_dir: PathBuf,
    app_log_dir: PathBuf,
    audio_dir: PathBuf,
    cache_dir: PathBuf,
    config_dir: PathBuf,
    data_dir: PathBuf,
    desktop_dir: PathBuf,
    document_dir: PathBuf,
    download_dir: PathBuf,
    executable_dir: PathBuf,
    font_dir: PathBuf,
    home_dir: PathBuf,
    local_data_dir: PathBuf,
    picture_dir: PathBuf,
    public_dir: PathBuf,
    resource_dir: PathBuf,
    runtime_dir: PathBuf,
    temp_dir: PathBuf,
    template_dir: PathBuf,
    data_path: PathBuf,
    local_res_path: PathBuf,
    local_data_path: PathBuf,
}

impl AppPaths {
    /// - **Windows**: `C:\Users\<User>\AppData\Local\<PackageName>`
    /// - **MacOS**: `/Users/<User>/Library/Caches/<PackageName>`
    pub fn app_cache_dir(&self) -> PathBuf {
        self.app_cache_dir.clone()
    }
    /// - **Windows**: `C:\Users\<User>\AppData\Roaming\<PackageName>`
    /// - **MacOS**: `/Users/<User>/Library/Application Support/<PackageName>`
    pub fn app_config_dir(&self) -> PathBuf {
        self.app_config_dir.clone()
    }
    /// - **Windows**: `C:\Users\<User>\AppData\Roaming\<PackageName>`
    /// - **MacOS**: `/Users/<User>/Library/Application Support/<PackageName>`
    pub fn app_data_dir(&self) -> PathBuf {
        self.app_data_dir.clone()
    }
    /// - **Windows**: `C:\Users\<User>\AppData\Local\<PackageName>`
    /// - **MacOS**: `/Users/<User>/Library/Application Support/<PackageName>`
    pub fn app_local_data_dir(&self) -> PathBuf {
        self.app_local_data_dir.clone()
    }
    /// - **Windows**: `C:\Users\<User>\AppData\Local\<PackageName>\logs`
    /// - **MacOS**: `/Users/<User>/Library/Logs/<PackageName>`
    pub fn app_log_dir(&self) -> PathBuf {
        self.app_log_dir.clone()
    }
    /// - **Windows**: `C:\Users\<User>\Music`
    /// - **MacOS**: `/Users/<User>/Music`
    pub fn audio_dir(&self) -> PathBuf {
        self.audio_dir.clone()
    }
    /// - **Windows**: `C:\Users\<User>\AppData\Local`
    /// - **MacOS**: `/Users/<User>/Library/Caches`
    pub fn cache_dir(&self) -> PathBuf {
        self.cache_dir.clone()
    }
    /// - **Windows**: `C:\Users\<User>\AppData\Roaming`
    /// - **MacOS**: `/Users/<User>/Library/Application Support`
    pub fn config_dir(&self) -> PathBuf {
        self.config_dir.clone()
    }
    /// - **Windows**: `C:\Users\<User>\AppData\Roaming`
    /// - **MacOS**: `/Users/<User>/Library/Application Support`
    pub fn data_dir(&self) -> PathBuf {
        self.data_dir.clone()
    }
    /// - **Windows**: `C:\Users\<User>\Desktop`
    /// - **MacOS**: `/Users/<User>/Desktop`
    pub fn desktop_dir(&self) -> PathBuf {
        self.desktop_dir.clone()
    }
    /// - **Windows**: `C:\Users\<User>\Documents`
    /// - **MacOS**: `/Users/<User>/Documents`
    pub fn document_dir(&self) -> PathBuf {
        self.document_dir.clone()
    }
    /// - **Windows**: `C:\Users\<User>\Downloads`
    /// - **MacOS**: `/Users/<User>/Downloads`
    pub fn download_dir(&self) -> PathBuf {
        self.download_dir.clone()
    }
    /// - **Windows**: `不支持`
    ///
    /// - **MacOS**: `不支持`
    pub fn executable_dir(&self) -> PathBuf {
        self.executable_dir.clone()
    }
    /// - **Windows**: `不支持`
    /// - **MacOS**: `/Users/<User>/Library/Fonts`
    pub fn font_dir(&self) -> PathBuf {
        self.font_dir.clone()
    }
    /// - **Windows**: `C:\Users\<User>`
    /// - **MacOS**: `/Users/<User>`
    pub fn home_dir(&self) -> PathBuf {
        self.home_dir.clone()
    }
    /// - **Windows**: `C:\Users\<User>\AppData\Local`
    /// - **MacOS**: `/Users/<User>/Library/Application Support`
    pub fn local_data_dir(&self) -> PathBuf {
        self.local_data_dir.clone()
    }
    /// - **Windows**: `C:\Users\<User>\Pictures`
    /// - **MacOS**: `/Users/<User>/Pictures`
    pub fn picture_dir(&self) -> PathBuf {
        self.picture_dir.clone()
    }
    /// - **Windows**: `C:\Users\Public`
    /// - **MacOS**: `/Users/<User>/Public`
    pub fn public_dir(&self) -> PathBuf {
        self.public_dir.clone()
    }
    /// - **Windows**: `\\?\E:\<ProjectPath>\src-rs\target\debug`
    /// - **MacOS**: `/Users/<ProjectPath>/src-rs/target/debug`
    pub fn resource_dir(&self) -> PathBuf {
        self.resource_dir.clone()
    }
    /// - **Windows**: `不支持`
    /// - **MacOS**: `不支持`
    pub fn runtime_dir(&self) -> PathBuf {
        self.runtime_dir.clone()
    }
    /// - std::env::temp_dir
    pub fn temp_dir(&self) -> PathBuf {
        self.temp_dir.clone()
    }
    /// - **Windows**: `C:\Users\<User>\AppData\Roaming\Microsoft\Windows\Templates`
    /// - **MacOS**: `不支持`
    pub fn template_dir(&self) -> PathBuf {
        self.template_dir.clone()
    }
    /// - **Windows**: `C:\Users\<User>\AppData\Roaming\<PackageName>\appdata`
    /// - **MacOS**: `/Users/<User>/Library/Application Support/<PackageName>/appdata`
    pub fn data_path(&self) -> PathBuf {
        self.data_path.clone()
    }
    /// - **Windows**: `\\?\E:\<ProjectPath>\src-rs\target\debug\res`
    /// - **MacOS**: `/Users/<ProjectPath>/src-rs/target/debug/res`
    pub fn local_res_path(&self) -> PathBuf {
        self.local_res_path.clone()
    }
    /// - **Windows**: `\\?\E:\<ProjectPath>\src-rs\target\debug\data`
    /// - **MacOS**: `/Users/<ProjectPath>/src-rs/target/debug/data`
    pub fn local_data_path(&self) -> PathBuf {
        self.local_data_path.clone()
    }
}

impl AppPaths {
    fn new() -> AppPaths {
        Self {
            ..Default::default()
        }
    }
    fn init(&mut self, handle: &AppHandle) {
        self.app_cache_dir = handle.path().app_cache_dir().unwrap_or_default();
        self.app_config_dir = handle.path().app_config_dir().unwrap_or_default();
        self.app_data_dir = handle.path().app_data_dir().unwrap_or_default();
        self.app_local_data_dir = handle.path().app_local_data_dir().unwrap_or_default();
        self.app_log_dir = handle.path().app_log_dir().unwrap_or_default();
        self.audio_dir = handle.path().audio_dir().unwrap_or_default();
        self.cache_dir = handle.path().cache_dir().unwrap_or_default();
        self.data_dir = handle.path().data_dir().unwrap_or_default();
        self.config_dir = handle.path().config_dir().unwrap_or_default();
        #[cfg(any(
            target_os = "linux",
            target_os = "windows",
            target_os = "macos",
            target_os = "ios"
        ))]
        {
            self.desktop_dir = handle.path().desktop_dir().unwrap_or_default();
            self.executable_dir = handle.path().executable_dir().unwrap_or_default();
            self.font_dir = handle.path().font_dir().unwrap_or_default();
            self.home_dir = handle.path().home_dir().unwrap_or_default();
            self.runtime_dir = handle.path().runtime_dir().unwrap_or_default();
            self.template_dir = handle.path().template_dir().unwrap_or_default();
        }
        #[cfg(any(target_os = "android",))]
        {
            self.desktop_dir = PathBuf::new();
            self.executable_dir = PathBuf::new();
            self.font_dir = PathBuf::new();
            self.home_dir = PathBuf::new();
            self.runtime_dir = PathBuf::new();
            self.template_dir = PathBuf::new();
        }
        self.document_dir = handle.path().document_dir().unwrap_or_default();
        self.download_dir = handle.path().download_dir().unwrap_or_default();
        self.local_data_dir = handle.path().local_data_dir().unwrap_or_default();
        self.picture_dir = handle.path().picture_dir().unwrap_or_default();
        self.public_dir = handle.path().public_dir().unwrap_or_default();
        self.resource_dir = handle.path().resource_dir().unwrap_or_default();
        self.data_path = self.app_data_dir().join("appdata");
        self.temp_dir = handle.path().temp_dir().unwrap_or_default();
        self.local_res_path = self.resource_dir().join("res");
        self.local_data_path = self.resource_dir().join("data");
    }
}

static APP_PATHS: LazyLock<Arc<RwLock<AppPaths>>> =
    LazyLock::new(|| Arc::new(RwLock::new(AppPaths::new())));

pub(crate) fn init_paths(app: &tauri::App) -> Result<()> {
    let handle = app.handle();
    let mut app_paths = APP_PATHS
        .write()
        .map_err(|_| anyhow!("write APP_PATHS error"))?;
    app_paths.init(handle);
    Ok(())
}

pub fn paths() -> RwLockReadGuard<'static, AppPaths> {
    APP_PATHS.read().expect("read APP_PATHS error")
}
