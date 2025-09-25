use crate::key_buffer::{Action, UKey};

impl Action {
    fn from_str(s: &str) -> Result<Action, &'static str> {
        match s {
            "up" => Ok(Action::Release),
            "down" => Ok(Action::Press),
            _ => Err("Unknown action"),
        }
    }
}

fn to_u_key(s: &str) -> Result<UKey, &'static str> {
    match s {
        "a" => Ok(UKey::A),
        "b" => Ok(UKey::B),
        "c" => Ok(UKey::C),
        "d" => Ok(UKey::D),
        "e" => Ok(UKey::E),
        "f" => Ok(UKey::F),
        "g" => Ok(UKey::G),
        "h" => Ok(UKey::H),
        "i" => Ok(UKey::I),
        "j" => Ok(UKey::J),
        "k" => Ok(UKey::K),
        "l" => Ok(UKey::L),
        "m" => Ok(UKey::M),
        "n" => Ok(UKey::N),
        "o" => Ok(UKey::O),
        "p" => Ok(UKey::P),
        "q" => Ok(UKey::Q),
        "r" => Ok(UKey::R),
        "s" => Ok(UKey::S),
        "t" => Ok(UKey::T),
        "u" => Ok(UKey::U),
        "v" => Ok(UKey::V),
        "w" => Ok(UKey::W),
        "x" => Ok(UKey::X),
        "y" => Ok(UKey::Y),
        "z" => Ok(UKey::Z),
        "esc" | "escape" => Ok(UKey::Esc),
        "leftctrl" | "lctrl" => Ok(UKey::LeftControl),
        "rightctrl" | "rctrl" => Ok(UKey::RightControl),
        "leftshift" | "lshift" => Ok(UKey::LeftShift),
        "rightshift" | "rshift" => Ok(UKey::RightShift),
        "leftalt" | "lalt" => Ok(UKey::LeftAlt),
        "rightalt" | "ralt" => Ok(UKey::RightAlt),
        "space" => Ok(UKey::Space),
        "tab" => Ok(UKey::Tab),
        "enter" | "return" => Ok(UKey::Enter),
        "backspace" => Ok(UKey::BackSpace),
        "capslock" => Ok(UKey::CapsLock),
        "f1" => Ok(UKey::F1),
        "f2" => Ok(UKey::F2),
        "f3" => Ok(UKey::F3),
        "f4" => Ok(UKey::F4),
        "f5" => Ok(UKey::F5),
        "f6" => Ok(UKey::F6),
        "f7" => Ok(UKey::F7),
        "f8" => Ok(UKey::F8),
        "f9" => Ok(UKey::F9),
        "f10" => Ok(UKey::F10),
        "f11" => Ok(UKey::F11),
        "f12" => Ok(UKey::F12),
        "insert" => Ok(UKey::Insert),
        "delete" => Ok(UKey::Delete),
        "home" => Ok(UKey::Home),
        "end" => Ok(UKey::End),
        "pageup" => Ok(UKey::PageUp),
        "pagedown" => Ok(UKey::PageDown),
        "up" => Ok(UKey::Up),
        "down" => Ok(UKey::Down),
        "left" => Ok(UKey::Left),
        "right" => Ok(UKey::Right),
        "numlock" => Ok(UKey::NumLock),
        "scrolllock" => Ok(UKey::ScrollLock),
        // "printscreen" => Ok(UKey::PrintScreen),
        // "pause" => Ok(UKey::Pause),
        // "menu" => Ok(UKey::Menu),
        "leftmeta" | "lmeta" | "leftwin" | "lwin" => Ok(UKey::LeftMeta),
        "rightmeta" | "rmeta" | "rightwin" | "rwin" => Ok(UKey::RightMeta),
        "0" => Ok(UKey::_0),
        "1" => Ok(UKey::_1),
        "2" => Ok(UKey::_2),
        "3" => Ok(UKey::_3),
        "4" => Ok(UKey::_4),
        "5" => Ok(UKey::_5),
        "6" => Ok(UKey::_6),
        "7" => Ok(UKey::_7),
        "8" => Ok(UKey::_8),
        "9" => Ok(UKey::_9),
        _ => Err("Unknown sedf"),
    }
}

#[derive(Debug, PartialEq)]
struct KeyExpr {
    key: UKey,
    action: Option<Action>,
}
#[derive(Debug, PartialEq)]
struct WaitExpr {
    milliseconds: u64,
}

#[derive(Debug, PartialEq)]
enum Expr {
    Key(KeyExpr),
    Wait(WaitExpr),
}

fn parse_expr(input: &str) -> Vec<Expr> {
    let input = input.to_lowercase();
    let mut exprs = Vec::<Expr>::new();
    for i in input.split('+') {
        let i = i.trim();
        let mut tmp_split: Vec<&str> = Vec::new();
        for sub_i in i.split(" ") {
            tmp_split.push(sub_i);
        }
        match tmp_split.len() {
            1 => {
                exprs.push(Expr::Key(KeyExpr {
                    key: to_u_key(tmp_split[0]).unwrap(),
                    action: None,
                }));
            }
            2 => match tmp_split[0] {
                "wait" => {
                    exprs.push(Expr::Wait(WaitExpr {
                        milliseconds: tmp_split[1].parse::<u64>().unwrap(),
                    }));
                }
                _ => {
                    exprs.push(Expr::Key(KeyExpr {
                        key: to_u_key(tmp_split[1]).unwrap(),
                        action: Some(Action::from_str(tmp_split[0]).unwrap()),
                    }));
                }
            },
            _ => {
                panic!("Unexpected number of elements in split: {:?}", tmp_split);
            }
        }
    }
    exprs
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parser() {
        macro_rules! assert_parsed_exprs {
            ($input:expr, $expected:expr) => {
                let exprs = parse_expr($input);
                assert_eq!(exprs, $expected);
            };
        }

        assert_parsed_exprs!(
            "Down leftctrl + Wait 500 + up leftctrl + wait 200 +      esc",
            vec![
                Expr::Key(KeyExpr {
                    key: UKey::LeftControl,
                    action: Some(Action::Press),
                }),
                Expr::Wait(WaitExpr { milliseconds: 500 }),
                Expr::Key(KeyExpr {
                    key: UKey::LeftControl,
                    action: Some(Action::Release),
                }),
                Expr::Wait(WaitExpr { milliseconds: 200 }),
                Expr::Key(KeyExpr {
                    key: UKey::Esc,
                    action: None,
                }),
            ]
        );

        // Additional tests
        assert_parsed_exprs!(
            "wait 1000 + up a + down b + c",
            vec![
                Expr::Wait(WaitExpr { milliseconds: 1000 }),
                Expr::Key(KeyExpr {
                    key: UKey::A,
                    action: Some(Action::Release),
                }),
                Expr::Key(KeyExpr {
                    key: UKey::B,
                    action: Some(Action::Press),
                }),
                Expr::Key(KeyExpr {
                    key: UKey::C,
                    action: None,
                }),
            ]
        );

        assert_parsed_exprs!(
            "esc",
            vec![Expr::Key(KeyExpr {
                key: UKey::Esc,
                action: None,
            })]
        );

        assert_parsed_exprs!(
            "down leftshift",
            vec![Expr::Key(KeyExpr {
                key: UKey::LeftShift,
                action: Some(Action::Press),
            })]
        );

        assert_parsed_exprs!(
            "wait 100000",
            vec![Expr::Wait(WaitExpr {
                milliseconds: 100_000
            })]
        );

        assert_parsed_exprs!("wait 50", vec![Expr::Wait(WaitExpr { milliseconds: 50 })]);

        let inp = "Down leftctrl + Wait 500 + up leftctrl + wait 200 +      esc";
        let exprs = parse_expr(&inp);
        for e in exprs {
            println!("Expressions {e:?}");
        }
    }
}
