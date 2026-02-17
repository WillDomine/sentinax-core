use std::io;
use rdev::{listen, Event, EventType};

pub fn start_tracking() -> io::Result<()>{
    println!("Init macOS Hook");
    // Start listening for events and handle them in the callback function
    if let Err(error) = listen(callback) {
        eprintln!("Error: {:?}", error);
    }
    return Ok(());
}
// Callback function to handle incoming events
fn callback(event: Event) {
    // Process the event based on its type
    match event.event_type {
        // Handle Mouse Move
        EventType::MouseMove { x, y } => {
            println!("Mouse Move: x={}, y={}", x, y);
        },
        // Handle Mouse Button Press
        EventType::ButtonPress(button) => {
            println!("Button Press: {:?}", button);
        },
        // Handle Key Press
        EventType::KeyPress(key) => {
            println!("Key Press: {:?}", key);
        },
        _ => {}
    }
}  