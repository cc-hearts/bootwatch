pub mod helper;
#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "windows")]
pub mod windows;

use helper::DisplayItem;

// Unified StartupType for cross-platform use
#[derive(Debug)]
pub enum StartupType {
    #[cfg(target_os = "macos")]
    Plist, // macOS Plist-based startup item
    #[cfg(target_os = "macos")]
    LoginItem, // macOS Login Item
    #[cfg(target_os = "windows")]
    Registry, // Windows Registry-based startup item
    #[cfg(target_os = "windows")]
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

/// 构建带展示信息（图标/类型标签/路径）的启动项列表
pub fn get_display_items() -> Vec<DisplayItem> {
    get_all_startup_items()
        .into_iter()
        .map(|item| {
            let (icon, type_label, value): (&str, &str, String) = match &item.item_type {
                #[cfg(target_os = "macos")]
                StartupType::Plist => ("📝", "Plist", item.path.clone().unwrap_or_default()),
                #[cfg(target_os = "macos")]
                StartupType::LoginItem => ("🚀", "Login Item", item.label.clone()),
                #[cfg(target_os = "windows")]
                StartupType::Registry => ("🔑", "Registry", item.label.clone()),
                #[cfg(target_os = "windows")]
                StartupType::StartupFolder => {
                    ("📂", "StartupFolder", item.path.clone().unwrap_or_default())
                }
            };
            DisplayItem {
                icon: icon.to_string(),
                type_label: type_label.to_string(),
                label: item.label.clone(),
                path: item.path.clone(),
                // option.label 内含类型标签，供各平台删除逻辑识别类型
                option: helper::OptionItem {
                    label: format!("{}: {}", type_label, item.label),
                    value,
                },
            }
        })
        .collect()
}

/// 删除指定启动项（跨平台入口）
pub fn delete_item(item: &helper::OptionItem) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    {
        macos::delete_startup_item(item)
    }
    #[cfg(target_os = "windows")]
    {
        windows::delete_startup_item(item)
    }
}
