mod key_buffer;
mod key_grabber;
mod udev_loop;
mod utils;

use key_buffer::KeyBuffer;
use std::error::Error;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Box<dyn Error>> {
    let uloop = udev_loop::Udev::new()?;
    let key_buffer = KeyBuffer::new()?;

    let buffer_cntr = Arc::new(key_buffer);
    udev_loop::Udev::start_listen(Arc::new(Mutex::new(uloop)), buffer_cntr.clone());
    return key_grabber::grab_kb_events(buffer_cntr.clone());
}
