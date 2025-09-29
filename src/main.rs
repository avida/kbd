use key_buffer::KeyBuffer;
use std::error::Error;
use std::sync::{Arc, Mutex};
use config::load_config;

mod config;
mod key_buffer;
mod key_grabber;
mod key_scheduler;
mod udev_loop;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let uloop = udev_loop::Udev::new().expect("Failed to create Udev device");
    let config = load_config();
    let key_buffer = KeyBuffer::new(config)?;

    let buffer_cntr = key_buffer.clone();

    udev_loop::Udev::start_listen(Arc::new(Mutex::new(uloop)), buffer_cntr.clone());
    return key_grabber::grab_kb_events(buffer_cntr.clone());
}
