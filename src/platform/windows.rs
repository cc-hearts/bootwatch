use crate::platform::helper::{parse_token, OptionItem};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE}; // Import HKCU and HKLM
use winreg::RegKey;

/// 启动项类型
#[derive(Debug)]
pub enum StartupType {
    Registry,      // Registry-based startup item
    StartupFolder, // Startup folder shortcut
}

/// 表示一个 Windows 启动项
#[derive(Debug)]
pub struct StartupItem {
    pub label: String,
    pub path: Option<String>,
    pub item_type: StartupType,
    /// 平台相关的删除令牌，格式：
    /// - Registry: `reg|<完整键路径>|<值名>`
    /// - StartupFolder: `file|<文件路径>`
    pub delete_value: String,
}

#[cfg(target_os = "windows")]
pub fn get_startup_apps() -> Vec<StartupItem> {
    let mut items = vec![];

    // 注册表启动项：同时记录 hive 名称，以便删除时构造完整键路径
    let reg_paths = vec![
        (
            HKEY_CURRENT_USER,
            "HKEY_CURRENT_USER",
            r"Software\Microsoft\Windows\CurrentVersion\Run",
        ),
        (
            HKEY_LOCAL_MACHINE,
            "HKEY_LOCAL_MACHINE",
            r"Software\Microsoft\Windows\CurrentVersion\Run",
        ),
    ];

    for (hive, hive_name, path) in reg_paths {
        if let Ok(key) = RegKey::predef(hive).open_subkey(path) {
            let full_key = format!("{}\\{}", hive_name, path);
            for name in key.enum_values().flatten().map(|(name, _)| name) {
                if let Ok(value) = key.get_value::<String, _>(&name) {
                    items.push(StartupItem {
                        delete_value: format!("reg|{}|{}", full_key, name),
                        label: name,
                        path: Some(value),
                        item_type: StartupType::Registry,
                    });
                }
            }
        }
    }

    // Check Startup folder
    if let Some(startup_dir) = get_startup_folder() {
        if startup_dir.exists() && startup_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(startup_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                            let path_str = path.display().to_string();
                            items.push(StartupItem {
                                delete_value: format!("file|{}", path_str),
                                label: file_name.to_string(),
                                path: Some(path_str),
                                item_type: StartupType::StartupFolder,
                            });
                        }
                    }
                }
            }
        }
    }

    items
}

/// Get the path to the user's Startup folder
fn get_startup_folder() -> Option<PathBuf> {
    std::env::var("APPDATA")
        .map(|appdata| {
            PathBuf::from(appdata)
                .join("Microsoft")
                .join("Windows")
                .join("Start Menu")
                .join("Programs")
                .join("Startup")
        })
        .ok()
}

/// 删除指定的开机启动项。
/// 参数：item - 要删除的启动项，其 `value` 为 `reg|<键路径>|<值名>` 或 `file|<文件路径>`。
#[cfg(target_os = "windows")]
pub fn delete_startup_item(item: &OptionItem) -> Result<(), Box<dyn std::error::Error>> {
    use encoding_rs::GBK;

    // 令牌格式：<kind>|<payload>
    let (kind, payload) = parse_token(&item.value);

    match kind {
        "reg" => {
            // payload = `<完整键路径>|<值名>`，仅按第一个 `|` 拆分，值名中可含 `|`
            let (key_path, value_name) = payload.split_once('|').ok_or("注册表删除令牌格式错误")?;
            let output = Command::new("reg")
                .args(["delete", key_path, "/v", value_name, "/f"])
                .output()?;

            if output.status.success() {
                Ok(())
            } else {
                let (decoded_stderr, _, _) = GBK.decode(&output.stderr);
                Err(format!(
                    "删除注册表启动项失败: {}\n错误: {}",
                    item.label,
                    decoded_stderr.trim()
                )
                .into())
            }
        }
        "file" => {
            fs::remove_file(payload)?;
            Ok(())
        }
        _ => Err(format!("不支持的启动项类型: {}", item.value).into()),
    }
}
