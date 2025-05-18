#[cfg(target_os = "windows")]
pub fn get_startup_apps() -> Result<(), Box<dyn std::error::Error>> {
    use winreg::enums::*;
    use winreg::RegKey;

    let hkey = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run")?;

    println!("Startup items from registry:");
    for (name, value) in hkey.enum_values().filter_map(Result::ok) {
        println!("{} => {:?}", name, value);
    }

    Ok(())
}
