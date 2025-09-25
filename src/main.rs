mod key_buffer;
mod key_grabber;
mod udev_loop;
mod utils;
mod key_scheduler;
mod config;

use key_buffer::KeyBuffer;
use std::error::Error;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn Error>> {
    let uloop = udev_loop::Udev::new().expect("Failed to create Udev device");
    let key_buffer = KeyBuffer::new()?;

    let buffer_cntr = key_buffer.clone();

    udev_loop::Udev::start_listen(Arc::new(Mutex::new(uloop)), buffer_cntr.clone());
    return key_grabber::grab_kb_events(buffer_cntr.clone());
}
