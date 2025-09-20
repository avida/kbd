#[macro_export]
macro_rules! evdev_to_uinput_key {
    ($key:expr) => {
        match $key {
            Key::KEY_A => UKey::A,
            Key::KEY_B => UKey::B,
            Key::KEY_C => UKey::C,
            Key::KEY_D => UKey::D,
            Key::KEY_E => UKey::E,
            Key::KEY_F => UKey::F,
            Key::KEY_G => UKey::G,
            Key::KEY_H => UKey::H,
            Key::KEY_I => UKey::I,
            Key::KEY_J => UKey::J,
            Key::KEY_K => UKey::K,
            Key::KEY_L => UKey::L,
            Key::KEY_M => UKey::M,
            Key::KEY_N => UKey::N,
            Key::KEY_O => UKey::O,
            Key::KEY_P => UKey::P,
            Key::KEY_Q => UKey::Q,
            Key::KEY_R => UKey::R,
            Key::KEY_S => UKey::S,
            Key::KEY_T => UKey::T,
            Key::KEY_U => UKey::U,
            Key::KEY_V => UKey::V,
            Key::KEY_W => UKey::W,
            Key::KEY_X => UKey::X,
            Key::KEY_Y => UKey::Y,
            Key::KEY_Z => UKey::Z,
            Key::KEY_1 => UKey::_1,
            Key::KEY_2 => UKey::_2,
            Key::KEY_3 => UKey::_3,
            Key::KEY_4 => UKey::_4,
            Key::KEY_5 => UKey::_5,
            Key::KEY_6 => UKey::_6,
            Key::KEY_7 => UKey::_7,
            Key::KEY_8 => UKey::_8,
            Key::KEY_9 => UKey::_9,
            Key::KEY_0 => UKey::_0,
            Key::KEY_ENTER => UKey::Enter,
            Key::KEY_ESC => UKey::Esc,
            Key::KEY_BACKSPACE => UKey::BackSpace,
            Key::KEY_TAB => UKey::Tab,
            Key::KEY_SPACE => UKey::Space,
            Key::KEY_MINUS => UKey::Minus,
            Key::KEY_EQUAL => UKey::Equal,
            Key::KEY_LEFTBRACE => UKey::LeftBrace,
            Key::KEY_RIGHTBRACE => UKey::RightBrace,
            Key::KEY_BACKSLASH => UKey::BackSlash,
            Key::KEY_SEMICOLON => UKey::SemiColon,
            Key::KEY_APOSTROPHE => UKey::Apostrophe,
            Key::KEY_GRAVE => UKey::Grave,
            Key::KEY_COMMA => UKey::Comma,
            Key::KEY_DOT => UKey::Dot,
            Key::KEY_SLASH => UKey::Slash,
            Key::KEY_CAPSLOCK => UKey::CapsLock,
            Key::KEY_F1 => UKey::F1,
            Key::KEY_F2 => UKey::F2,
            Key::KEY_F3 => UKey::F3,
            Key::KEY_F4 => UKey::F4,
            Key::KEY_F5 => UKey::F5,
            Key::KEY_F6 => UKey::F6,
            Key::KEY_F7 => UKey::F7,
            Key::KEY_F8 => UKey::F8,
            Key::KEY_F9 => UKey::F9,
            Key::KEY_F10 => UKey::F10,
            Key::KEY_F11 => UKey::F11,
            Key::KEY_F12 => UKey::F12,
            Key::KEY_LEFTCTRL => UKey::LeftControl,
            Key::KEY_RIGHTCTRL => UKey::RightControl,
            Key::KEY_LEFTSHIFT => UKey::LeftShift,
            Key::KEY_RIGHTSHIFT => UKey::RightShift,
            Key::KEY_LEFTALT => UKey::LeftAlt,
            Key::KEY_RIGHTALT => UKey::RightAlt,
            Key::KEY_LEFTMETA => UKey::LeftMeta,
            Key::KEY_RIGHTMETA => UKey::RightMeta,
            Key::KEY_HOME => UKey::Home,
            Key::KEY_END => UKey::End,
            Key::KEY_PAGEUP => UKey::PageUp,
            Key::KEY_PAGEDOWN => UKey::PageDown,
            Key::KEY_INSERT => UKey::Insert,
            Key::KEY_DELETE => UKey::Delete,
            Key::KEY_UP => UKey::Up,
            Key::KEY_DOWN => UKey::Down,
            Key::KEY_LEFT => UKey::Left,
            Key::KEY_RIGHT => UKey::Right,
            Key::KEY_NUMLOCK => UKey::NumLock,
            Key::KEY_SCROLLLOCK => UKey::ScrollLock,
            Key::KEY_SYSRQ => UKey::SysRq,
            Key::KEY_LINEFEED => UKey::LineFeed,
            Key::KEY_F13 => UKey::F13,
            Key::KEY_F14 => UKey::F14,
            Key::KEY_F15 => UKey::F15,
            Key::KEY_F16 => UKey::F16,
            Key::KEY_F17 => UKey::F17,
            Key::KEY_F18 => UKey::F18,
            Key::KEY_F19 => UKey::F19,
            Key::KEY_F20 => UKey::F20,
            Key::KEY_F21 => UKey::F21,
            Key::KEY_F22 => UKey::F22,
            Key::KEY_F23 => UKey::F23,
            Key::KEY_F24 => UKey::F24,
            // Add more mappings as needed
            _ => {
                // If not mapped, fallback to UKey::A (or handle as needed)
                UKey::A
            }
        }
    };
}

#[macro_export]
macro_rules! mmm {
    () => {
        println!("------ this is macro ------")
    };
}

#[cfg(test)]
mod tests {
    use evdev::Key;
    use uinput::event::keyboard::Key as UKey;

    #[test]
    fn test_evdev_to_uinput_key_basic() {
        assert_eq!(evdev_to_uinput_key!(Key::KEY_A), UKey::A);
        assert_eq!(evdev_to_uinput_key!(Key::KEY_B), UKey::B);
        assert_eq!(evdev_to_uinput_key!(Key::KEY_1), UKey::_1);
        assert_eq!(evdev_to_uinput_key!(Key::KEY_ENTER), UKey::Enter);
        assert_eq!(evdev_to_uinput_key!(Key::KEY_F12), UKey::F12);
    }

    #[test]
    fn test_evdev_to_uinput_key_default() {
        assert_eq!(evdev_to_uinput_key!(Key::KEY_UNKNOWN), UKey::A);
    }
}
