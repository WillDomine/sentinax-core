use chrono::Local;
use evdev::Device;
use evdev::EventType;
use std::fs;
use std::io;
use std::thread;

pub fn start_tracking() -> io::Result<()> {
    println!("Init Linux Hook");

    //Allocate Devices List
    let mut threads = Vec::new();
    //Open inputs path for scanning
    let paths = fs::read_dir("/dev/input")?;

    for path in paths {
        let path = path?.path();

        if let Ok(device) = Device::open(&path) {
            let name = device.name().unwrap_or("Unknown").to_string();

            println!("Attached: {} ({:?})", name, path);
            threads.push(thread::spawn(move || {
                monitor_device(device, name);
            }));
        }
    }

    if threads.is_empty() {
        println!("No Devices Found");
        return Ok(());
    }

    println!("Running Sentinax Core");

    for t in threads {
        let _ = t.join();
    }

    return Ok(());
}

fn monitor_device(mut device: Device, name: String) {
    loop {
        match device.fetch_events() {
            Ok(iterator) => {
                for event in iterator {
                    let now = Local::now();
                    let timestamp = now.format("%H:%M:%S%.3f");

                    match event.event_type() {
                        // Handle Keyboard and Mouse Button Events
                        EventType::KEY => {
                            println!(
                                "{} | {} | KEY | Code: {} | State {}",
                                timestamp,
                                name,
                                event.code(),
                                event.value()
                            );
                        }
                        // Handle Mouse Movement Events
                        EventType::RELATIVE => {
                            let axis = if event.code() == 0 { "X" } else { "Y" };
                            println!(
                                "{} | {} | Move | Axis: {} | Delta {}",
                                timestamp,
                                name,
                                axis,
                                event.value()
                            );
                        }
                        // Handle Absolute Position Events (e.g., Touchscreens, Controllers)
                        EventType::ABSOLUTE => {
                            println!(
                                "{} | {} | ABS   | Code: {} | Val: {}",
                                timestamp,
                                name,
                                event.code(),
                                event.value()
                            );
                        }
                        _ => {}
                    }
                }
            }
            Err(e) => {
                println!("Device {} Disconnected: {:?}", name, e);
            }
        }
    }
}
