#[cfg(target_os = "linux")]
pub mod linux;

pub fn run() {
    #[cfg(target_os = "linux")]
    {
        if let Err(e) = linux::start_tracking() {
            eprintln!("Linux Hook Failed: {}", e);
        }
    }
}

