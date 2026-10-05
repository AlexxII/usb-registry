use std::path::PathBuf;

pub fn database_path() -> PathBuf {
    PathBuf::from("app.db")
}

#[cfg(target_os = "linux")]
pub mod os_linux;
#[cfg(target_os = "linux")]
pub use os_linux::get_current_usb_flash_drives;
#[cfg(target_os = "linux")]
pub use os_linux::get_history_usb_from_os;

#[cfg(target_os = "windows")]
mod os_window;
#[cfg(target_os = "windows")]
pub use os_window::get_current_usb_flash_drives;
#[cfg(target_os = "windows")]
pub use os_window::get_history_usb_from_os;
