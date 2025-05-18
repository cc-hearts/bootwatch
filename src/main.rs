mod platform;
use crate::platform::macos::StartupType;

fn main() {
    println!("BootWatch - startup item scanner\n");

    let items = platform::get_all_startup_items();

    println!("📦 共发现 {} 个开机启动项：\n", items.len());
    for item in items {
        match item.item_type {
            StartupType::Plist => {
                println!("📝 {} ({})", item.label, item.path.as_deref().unwrap_or("未知路径"));
            }
            StartupType::LoginItem => {
                println!("🚀 {} (Login Item)", item.label);
            }
        }
    }
}
