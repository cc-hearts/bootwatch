use crate::platform::helper::OptionItem;
use shellexpand;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// 启动项类型
#[derive(Debug)]
pub enum StartupType {
    Plist,
    LoginItem,
}

/// 表示一个 macOS 启动项
#[derive(Debug)]
pub struct StartupItem {
    pub label: String,
    pub path: Option<String>, // LoginItem 可能没有路径
    pub item_type: StartupType,
}

/// 获取 macOS 启动项（LaunchAgents & LaunchDaemons）
#[cfg(target_os = "macos")]
pub fn get_startup_apps() -> Vec<StartupItem> {
    let mut items = vec![];

    // 要检查的目录
    let dirs = vec![
        "~/Library/LaunchAgents", // 用户级
        "/Library/LaunchAgents",  // 系统用户环境
        "/Library/LaunchDaemons", // 系统服务
    ];

    for dir in dirs {
        let expanded_path: PathBuf = shellexpand::tilde(dir).into_owned().into();
        if expanded_path.exists() && expanded_path.is_dir() {
            if let Ok(entries) = fs::read_dir(expanded_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|e| e.to_str()) == Some("plist") {
                        if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
                            items.push(StartupItem {
                                label: file_name.to_string(),
                                path: Some(path.display().to_string()),
                                item_type: StartupType::Plist,
                            });
                        }
                    }
                }
            }
        }
    }

    items
}

/// 获取 macOS 登录项（Login Items）
#[cfg(target_os = "macos")]
pub fn get_login_items() -> Vec<StartupItem> {
    let output = Command::new("osascript")
        .arg("-e")
        .arg(r#"tell application "System Events" to get the name of every login item"#)
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                let stdout = String::from_utf8_lossy(&result.stdout);
                stdout
                    .split(", ")
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .map(|label| StartupItem {
                        label,
                        path: None,
                        item_type: StartupType::LoginItem,
                    })
                    .collect()
            } else {
                eprintln!("osascript error: {:?}", result.stderr);
                vec![]
            }
        }
        Err(err) => {
            eprintln!("Failed to run osascript: {}", err);
            vec![]
        }
    }
}

/// 合并获取所有启动项
#[cfg(target_os = "macos")]
pub fn get_all_startup_items() -> Vec<StartupItem> {
    let mut all = vec![];
    all.extend(get_startup_apps());
    all.extend(get_login_items());
    all
}

/// 删除指定的开机启动项。
/// 参数：item - 要删除的启动项。
#[cfg(target_os = "macos")]
pub fn delete_startup_item(item: &OptionItem) -> Result<(), Box<dyn std::error::Error>> {
    use std::process::Command;

    match item.value.as_str() {
        value if value.ends_with(".plist") => {
            // 对于 Plist 文件，使用 launchctl 卸载并删除文件
            let unload_cmd = Command::new("launchctl")
                .arg("unload")
                .arg(&item.value)
                .output()
                .expect("Failed to unload plist");

            if unload_cmd.status.success() {
                std::fs::remove_file(&item.value).expect("Failed to delete plist file");
                println!("成功删除 Plist 启动项: {}", item.value);
            } else {
                return Err("卸载 Plist 失败".into());
            }
        }
        _ if item.label.contains("Login Item") => {
            // 对于 Login Item，使用 osascript（AppleScript）移除
            // 这是一个简化示例，实际可能需要更多处理
            let script = format!(
                r#"tell application "System Events" to delete login item "{}""#,
                item.value
            );
            let output = Command::new("osascript")
                .arg("-e")
                .arg(script)
                .output()
                .expect("Failed to delete login item");

            if output.status.success() {
                println!("成功删除 Login Item: {}", item.value);
            } else {
                return Err("删除 Login Item 失败".into());
            }
        }
        _ => {
            return Err("不支持的启动项类型".into());
        }
    }
    Ok(())
}
