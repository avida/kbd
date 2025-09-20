use evdev::{Device, InputEventKind, Key};
use std::error::Error;
use std::thread;
use uinput::event::keyboard::Key as UKey;

mod evdev_to_input;
use super::kdb_processor::{Action, ThreadBuffer};

pub use crate::evdev_to_uinput_key;

const DEVICE_PATH: &str = "/dev/input/event3";

fn safe_thread() {
    thread::spawn(move || {
        thread::sleep(std::time::Duration::from_secs(20));
        println!("safe thread exit");
        std::process::exit(0);
    });
}
pub fn grub() -> Result<(), Box<dyn Error>> {
    // safe_thread();
    let mut buffer = ThreadBuffer::new();

    let mut dev = Device::open(DEVICE_PATH)?;
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
                if uinput_key == UKey::Esc {
                    std::process::exit(0);
                }
                if event.value() == 0 || event.value() == 1 {
                    let action = if event.value() == 0 {
                        Action::Release
                    } else {
                        Action::Press
                    };
                    println!("push event {:?} {:?}", uinput_key, action);
                    buffer.push(uinput_key, action);
                }
                // match event.value() {
                //     1 => uinput_dev.press(&uinput_key)?,
                //     0 => uinput_dev.release(&uinput_key)?,
                //     _ => {
                //         println!("Unknow value {:?}", event.value());
                //     }
                // }
                // uinput_dev.synchronize()?;
            } else {
                // println!("Unknow key {:?}", event);
            }
        }

        while let Some(event) = buffer.try_pop() {
            println!("send event {:?}", event);
            match event.action {
                Action::Press => uinput_dev.press(&event.key)?,
                Action::Release => uinput_dev.release(&event.key)?,
            }
        }
        uinput_dev.synchronize()?;
    }
}
