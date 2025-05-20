#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

// Unified StartupType for cross-platform use
#[derive(Debug)]
pub enum StartupType {
    Plist,         // macOS Plist-based startup item
    LoginItem,     // macOS Login Item
    Registry,      // Windows Registry-based startup item
    StartupFolder, // Windows Startup folder shortcut
}

// Unified StartupItem for cross-platform use
#[derive(Debug)]
pub struct StartupItem {
    pub label: String,
    pub path: Option<String>,
    pub item_type: StartupType,
}

#[cfg(target_os = "macos")]
pub fn get_all_startup_items() -> Vec<StartupItem> {
    macos::get_all_startup_items()
        .into_iter()
        .map(|item| StartupItem {
            label: item.label,
            path: item.path,
            item_type: match item.item_type {
                macos::StartupType::Plist => StartupType::Plist,
                macos::StartupType::LoginItem => StartupType::LoginItem,
            },
        })
        .collect()
}

#[cfg(target_os = "windows")]
pub fn get_all_startup_items() -> Vec<StartupItem> {
    windows::get_startup_apps()
        .into_iter()
        .map(|item| StartupItem {
            label: item.label,
            path: item.path,
            item_type: match item.item_type {
                windows::StartupType::Registry => StartupType::Registry,
                windows::StartupType::StartupFolder => StartupType::StartupFolder,
            },
        })
        .collect()
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn get_all_startup_items() -> Vec<StartupItem> {
    vec![]
}
