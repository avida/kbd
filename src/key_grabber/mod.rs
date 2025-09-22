use evdev::{Device, InputEventKind, Key};
use std::error::Error;
use std::sync::Arc;
use uinput::event::keyboard::Key as UKey;

mod evdev_to_input;

use crate::key_buffer::{Action, KeyBuffer};

use crate::debug_println;
pub use crate::evdev_to_uinput_key;

const DEVICE_PATH: &str = "/dev/input/event3";

pub fn grab_kb_events(buffer: Arc<KeyBuffer>) -> Result<(), Box<dyn Error>> {
    // Auto exit after 20 seconds, safety measure to not dead lock keyboard input
    #[cfg(debug_assertions)]
    std::thread::spawn(move || {
        const EXIT_S: u64 = 30;
        println!("Start safe thread, will exit in {} seconds", EXIT_S);
        std::thread::sleep(std::time::Duration::from_secs(EXIT_S));
        println!("safe thread exit");
        std::process::exit(0);
    });

    let mut dev = Device::open(DEVICE_PATH)?;
    dev.grab()?;
    loop {
        for event in dev.fetch_events()? {
            if let InputEventKind::Key(key) = event.kind() {
                let uinput_key: UKey = evdev_to_uinput_key!(key);
                debug_println!("evdev {:?} {}", uinput_key, event.value());
                #[cfg(debug_assertions)]
                if uinput_key == UKey::Esc {
                    std::process::exit(0);
                }
                if event.value() == 0 || event.value() == 1 {
                    let action = if event.value() == 0 {
                        Action::Release
                    } else {
                        Action::Press
                    };
                    buffer.push(uinput_key, action);
                }
            } else {
            }
        }
    }
}
