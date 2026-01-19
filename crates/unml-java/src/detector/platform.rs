#[cfg(windows)]
pub mod windows;
#[cfg(windows)]
pub use windows::WindowsDetector;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use macos::MacOSDetector;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::LinuxDetector;
