use std::env;
/// 获取当前运行平台的操作系统名称
///
/// 返回一个静态生命周期的字符串切片，表示当前操作系统类型
/// 可能的返回值包括: "macos", "windows", "linux" 等
///
/// # Examples
///
/// ```
/// let current_os = get_current_os();
/// println!("当前操作系统: {}", current_os);
/// ```
#[must_use]
pub const fn get_current_os() -> &'static str {
    env::consts::OS
}
