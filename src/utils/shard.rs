use std::env;

pub fn get_current_os() -> String {
    let os = env::consts::OS;
    os.to_string()
}
