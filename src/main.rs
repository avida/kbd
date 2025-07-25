use evdev::{Device, InputEventKind, Key};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use uinput::event::keyboard::Key as UKey;

mod evdev_to_input;

fn print_some_line(device_path: &str) {
    println!("This is some line");
    println!("Listening on: {}", device_path);
}

fn main() -> Result<(), Box<dyn Error>> {
    // Change this to your actual device path
    let device_path = "/dev/input/event3";

    let mut dev = Device::open(device_path)?;
    dev.grab()?;

    let mut uinput_dev = uinput::default()?
        .name("remapper")?
        .event(uinput::event::Keyboard::All)?
        .create()?;

    print_some_line(device_path);

    loop {
        let mut all_events = Vec::new();
        for ev in dev.fetch_events()? {
            all_events.push(ev);
        }
        for event in &all_events {
            if let InputEventKind::Key(key) = event.kind() {
                let uinput_key = evdev_to_uinput_key!(key);
                match event.value() {
                    1 => uinput_dev.press(&uinput_key)?,
                    0 => uinput_dev.release(&uinput_key)?,
                    _ => {
                        // println!("Unknow value {:?}", event.value());
                    }
                }
                uinput_dev.synchronize()?;
            } else {
                // println!("Unknow key {:?}", event);
            }
        }
        // Check if all_events contains both F23 and Meta keys pressed
        let mut has_f23 = false;
        let mut has_meta = false;
        let mut has_lshift = false;

        for event in &all_events {
            if let InputEventKind::Key(key) = event.kind() {
                if key == Key::KEY_F23 && event.value() == 0 {
                    has_f23 = true;
                }
                // Meta can be KEY_LEFTMETA or KEY_RIGHTMETA
                if key == Key::KEY_LEFTMETA && event.value() == 0 {
                    has_meta = true;
                }
                if key == Key::KEY_LEFTSHIFT && event.value() == 0 {
                    has_lshift = true;
                }
            }
        }

        if has_f23 && has_meta && has_lshift {
            // Spawn a new thread to print "a" after 500 milliseconds, so as not to block the loop
            // Emulate sticky left control key: press, wait, release
            std::thread::scope(|s| {
                s.spawn(|| {
                    // Press left control
                    println!("Gotcha");
                    let _ = uinput_dev.press(&UKey::LeftControl);
                    uinput_dev.synchronize();
                    sleep(Duration::from_millis(2000));
                    println!("finished");
                    // Release left control
                    uinput_dev.release(&UKey::LeftControl);
                    uinput_dev.synchronize();
                });
                println!("release");
            });
        }

        sleep(Duration::from_millis(1));
    }
}
