mod platform;
use crate::platform::helper::OptionItem;
use crate::platform::StartupType;
use dialoguer::{theme::ColorfulTheme, Select};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("BootWatch - startup item scanner\n");

    let items = platform::get_all_startup_items();

    println!("📦 共发现 {} 个开机启动项：\n", items.len());
    let mut select_ret: Vec<OptionItem> = vec![];
    for item in items {
        match item.item_type {
            #[cfg(target_os = "macos")]
            StartupType::Registry => {
                select_ret.push(OptionItem {
                    label: format!(
                        "📝 {} ({})",
                        item.label,
                        item.path.as_deref().unwrap_or("未知路径")
                    ),
                    value: item.path.as_deref().unwrap_or("").to_string(),
                });
            }
            
            #[cfg(target_os = "macos")]
            StartupType::StartupFolder => {
                select_ret.push(OptionItem {
                    label: format!("🚀 {} (Login Item)", item.label,),
                    value: item.label,
                });
            }
            #[cfg(target_os = "windows")]
            StartupType::Registry => {
                select_ret.push(OptionItem {
                    label: format!(
                        "🔑 {} (Registry, {})",
                        item.label,
                        item.path.as_deref().unwrap_or("未知路径")
                    ),

                    value:item.label,
                });
            }

            #[cfg(target_os = "windows")]
            StartupType::StartupFolder => {
                select_ret.push(OptionItem {
                    label: format!(
                        "📂 {} (Startup Folder, {})",
                        item.label,
                        item.path.as_deref().unwrap_or("未知路径")
                    ),
                    value: item.path.as_deref().unwrap_or("").to_string(),
                });
            }
        }
    }

    let labels: Vec<&str> = select_ret.iter().map(|item| &item.label[..]).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("请选择一个选项")
        .items(&labels)
        .default(0)
        .interact()?;

    let selected = &select_ret[selection];
    println!("你选择了: {} (值: {})", selected.label, selected.value);

    #[cfg(target_os = "macos")]
    crate::platform::macos::delete_startup_item(selected).unwrap();

    #[cfg(target_os = "windows")]
    crate::platform::windows::delete_startup_item(selected).unwrap();

    Ok(())
}
