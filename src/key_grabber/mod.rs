use evdev::{Device, InputEventKind, Key};
use std::error::Error;
// use std::thread::sleep;
// use std::time::Duration;
use uinput::event::keyboard::Key as UKey;

mod evdev_to_input;

pub use crate::evdev_to_uinput_key;
pub use crate::mmm;

pub fn grub() -> Result<(), Box<dyn Error>> {
    mmm!();

    let device_path = "/dev/input/event3";
    let mut dev = Device::open(device_path)?;
    dev.grab()?;
    let mut uinput_dev = uinput::default()?
        .name("remapper")?
        .event(uinput::event::Keyboard::All)?
        .create()?;

    loop {
        for event in dev.fetch_events()? {
            if let InputEventKind::Key(key) = event.kind() {
                let uinput_key: UKey = evdev_to_uinput_key!(key);
                println!("key event {:?}", uinput_key);
                match event.value() {
                    1 => uinput_dev.press(&uinput_key)?,
                    0 => uinput_dev.release(&uinput_key)?,
                    _ => {
                        println!("Unknow value {:?}", event.value());
                    }
                }
                uinput_dev.synchronize()?;
            } else {
                // println!("Unknow key {:?}", event);
            }
        }
    }
}