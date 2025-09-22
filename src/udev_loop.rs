use super::key_buffer::KeyBuffer;
use crate::key_buffer::{Action, Event};
use Result;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;

type Res = Result<(), Box<dyn Error>>;
pub type ALoop = Arc<Mutex<Udev>>;
use crate::debug_println;

pub struct Udev {
    device: uinput::Device,
}

impl Udev {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let uinput_dev = uinput::default()?
            .name("remapper")?
            .event(uinput::event::Keyboard::All)?
            .create()?;

        Ok(Udev { device: uinput_dev })
    }
    pub fn send_event(&mut self, event: Event) -> Res {
        debug_println!("Send event {:?}", event);
        match event.action {
            Action::Press => self.device.press(&event.key)?,
            Action::Release => self.device.release(&event.key)?,
        }
        Ok(())
    }

    pub fn sync(&mut self) -> Res {
        self.device.synchronize()?;
        Ok(())
    }
    pub fn start_listen(udev: ALoop, buffer: Arc<KeyBuffer>) {
        thread::spawn(move || {
            let mut this = udev.lock().unwrap();
            loop {
                if let Some(event) = buffer.pop() {
                    debug_println!("Send {:?}", event);
                    this.send_event(event).unwrap();
                    this.sync().unwrap();
                }
            }
        });
    }
}
