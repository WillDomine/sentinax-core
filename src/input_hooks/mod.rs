#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

pub fn run() {
    #[cfg(target_os = "linux")]
    {
        if let Err(e) = linux::start_tracking() {
            eprintln!("Linux Hook Failed: {}", e);
        }
    }
    #[cfg(target_os = "windows")]
    {
        if let Err(e) = windows::start_tracking() {
            eprintln!("Windows Hook Failed: {}", e);
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Err(e) = macos::start_tracking() {
            eprintln!("macOS Hook Failed: {}", e);  
        }
    }
}

