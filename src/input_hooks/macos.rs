use std::io;
use rdev::{listen, Event, EventType};

pub fn start_tracking() -> io::Result<()>{
    println!("Init macOS Hook");
    if let Err(error) = listen(callback) {
        eprintln!("Error: {:?}", error);
    }
    return Ok(());
}

fn callback(event: Event) {
    match event.event_type {
        EventType::MouseMove { x, y } => {
            println!("Mouse Move: x={}, y={}", x, y);
        },
        EventType::ButtonPress(button) => {
            println!("Button Press: {:?}", button);
        },
        EventType::KeyPress(key) => {
            println!("Key Press: {:?}", key);
        },
        _ => {}
    }
}  