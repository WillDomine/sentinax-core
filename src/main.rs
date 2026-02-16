use rdev::{listen, Event, EventType};
use chrono::Local;

fn main() {
    println!("Starting Sentinax...");

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event) {
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    match event.event_type {
        EventType::MouseMove { x, y } => {
            println!("{},MOVE,{:.1},{:.1},0", timestamp, x, y);
        }
        EventType::ButtonPress(button) => {
            println!("{},CLICK,0,0,{:?}", timestamp, button);
        }
        EventType::KeyPress(key) => {
            println!("{},KEY,0,0,{:?}", timestamp, key);
        }
        _ => {}
    }
}
