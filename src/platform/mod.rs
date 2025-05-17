// src/platform/mod.rs

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "macos")]
pub mod macos;

