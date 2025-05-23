use crate::platform::helper::OptionItem;
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
}

#[cfg(target_os = "windows")]
pub fn get_startup_apps() -> Vec<StartupItem> {
    let mut items = vec![];

    // Check registry startup items
    let reg_paths = vec![
        (
            HKEY_CURRENT_USER,
            r"Software\Microsoft\Windows\CurrentVersion\Run",
        ),
        (
            HKEY_LOCAL_MACHINE,
            r"Software\Microsoft\Windows\CurrentVersion\Run",
        ),
    ];

    for (hive, path) in reg_paths {
        if let Ok(key) = RegKey::predef(hive).open_subkey(path) {
            for name in key.enum_values().flatten().map(|(name, _)| name) {
                if let Ok(value) = key.get_value::<String, _>(&name) {
                    items.push(StartupItem {
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
                            items.push(StartupItem {
                                label: file_name.to_string(),
                                path: Some(path.display().to_string()),
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

#[cfg(target_os = "windows")]
pub fn delete_startup_item(item: &OptionItem) -> Result<(), Box<dyn std::error::Error>> {
    // 构建要执行的 reg delete 命令
    let command = format!(
        r#"reg delete HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Run /v {} /f"#,
        item.origin_label
    );

    // 执行命令
    let output = Command::new("cmd").args(&["/C", &command]).output()?;

    // 检查命令执行结果
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("执行命令成功: {}", command);
        println!("命令输出: {}", stdout);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("执行命令失败: {}\n错误: {}", command, stderr).into())
    }
}
