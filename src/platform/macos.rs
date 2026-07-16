use crate::platform::helper::{parse_token, OptionItem};
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
    /// 平台相关的删除令牌，格式：
    /// - Plist: `plist|<文件路径>`
    /// - LoginItem: `loginitem|<名称>`
    pub delete_value: String,
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
                            let path_str = path.display().to_string();
                            items.push(StartupItem {
                                label: file_name.to_string(),
                                path: Some(path_str.clone()),
                                item_type: StartupType::Plist,
                                delete_value: format!("plist|{}", path_str),
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
                        delete_value: format!("loginitem|{}", label),
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

/// 转义 AppleScript 字符串中的特殊字符，避免注入
#[cfg(target_os = "macos")]
fn escape_applescript_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

/// 删除指定的开机启动项。
/// 参数：item - 要删除的启动项，其 `value` 为 `plist|<路径>` 或 `loginitem|<名称>`。
#[cfg(target_os = "macos")]
pub fn delete_startup_item(item: &OptionItem) -> Result<(), Box<dyn std::error::Error>> {
    // 令牌格式：<kind>|<payload>，其中 payload 可能自身包含 `|`（如路径/名称）
    let (kind, payload) = parse_token(&item.value);

    match kind {
        "plist" => {
            let path = payload;
            // 先卸载，再删除文件；任一步失败均向上返回错误而非 panic
            let unload = Command::new("launchctl").arg("unload").arg(path).output()?;
            if !unload.status.success() {
                let err = String::from_utf8_lossy(&unload.stderr);
                return Err(format!("卸载 Plist 失败 ({}): {}", item.label, err.trim()).into());
            }
            fs::remove_file(path)?;
        }
        "loginitem" => {
            let name = payload;
            let escaped = escape_applescript_string(name);
            let script = format!(
                r#"tell application "System Events" to delete login item "{}""#,
                escaped
            );
            let output = Command::new("osascript").arg("-e").arg(script).output()?;
            if !output.status.success() {
                let err = String::from_utf8_lossy(&output.stderr);
                return Err(
                    format!("删除 Login Item 失败 ({}): {}", item.label, err.trim()).into(),
                );
            }
        }
        _ => {
            return Err(format!("不支持的启动项类型: {}", item.value).into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::escape_applescript_string;

    #[test]
    fn escape_quotes_and_backslashes() {
        assert_eq!(escape_applescript_string(r#"a"b"#), r#"a\"b"#);
        assert_eq!(escape_applescript_string(r"a\b"), r"a\\b");
    }

    #[test]
    fn escape_plain_string_unchanged() {
        assert_eq!(escape_applescript_string("Dropbox"), "Dropbox");
    }
}
