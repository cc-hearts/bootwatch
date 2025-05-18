// src/platform/mod.rs

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os="macos")]
pub use macos::get_all_startup_items;
