mod input_hooks;

fn main() {
    println!("Starting Sentinax...");
    // Start tracking input events based on the operating system
    input_hooks::run();
}

