#![allow(dead_code)]
use std::collections::HashSet;

use crate::config::parser::Expr;
use crate::key_buffer::{Action, Event};
pub use config_processor::{get_action, action_to_events};
pub use parser::Expressions;
use parser::{parse_expr};
use serde::Deserialize;
use toml::Table;

mod config_processor;
mod parser;

#[derive(Deserialize, Debug)]
struct Config {
    delay_ms: Option<u64>,
    main: Table,
}

#[derive(Debug)]
pub struct KeyCombination {
    combination: Expressions,
    action: Expressions,
}

#[derive(Debug)]
pub struct KeyCombinationHashed {
    combinations: KeyCombination,
    keys_hashes: Box<KeyHashes>,
}

type KeyHashes = HashSet<u64>;
#[derive(Debug)]
pub struct ParsedConfig {
    pub delay_ms: Option<u64>,
    pub key_combinations: Vec<KeyCombinationHashed>,
    combo_hashes: Box<KeyHashes>,
}

impl ParsedConfig {
    pub fn has_key(&self, event: &Event) -> bool {
        let hash = event.get_u64_hash();
        self.combo_hashes.contains(&hash)
    }
}

#[allow(dead_code)]
fn read_config() -> Result<Config, &'static str> {
    #[cfg(debug_assertions)]
    let config_path = "config.toml";
    #[cfg(not(debug_assertions))]
    let config_path = "/etc/kbd/config.toml";
    _read_config(config_path)
}

pub fn load_config() -> ParsedConfig {

    let raw_config = read_config().unwrap();
    _parse_config(&raw_config)
}

fn _parse_config(config: &Config) -> ParsedConfig {
    let mut combos = Vec::<KeyCombinationHashed>::new();
    let mut total_hashes = Box::new(KeyHashes::new());
    for (k, v) in config.main.iter() {
        let mut key_events = Box::new(KeyHashes::new());
        if let toml::Value::String(v) = v {
            let parsed_condition = parse_expr(k);

            for c in &parsed_condition {
                if let Expr::Key(k) = c {
                    match &k.action {
                        None => {
                            let hash = Event {
                                key: k.key,
                                action: Action::Press,
                            }
                            .get_u64_hash();

                            key_events.insert(hash);
                            total_hashes.insert(hash);
                            let hash = Event {
                                key: k.key,
                                action: Action::Release,
                            }
                            .get_u64_hash();
                            key_events.insert(hash);
                            total_hashes.insert(hash);
                        }
                        Some(action) => {
                            let hash = Event {
                                key: k.key,
                                action: action.clone(),
                            }
                            .get_u64_hash();
                            key_events.insert(hash);
                            total_hashes.insert(hash);
                        }
                    }
                }
            }

            let parsed_action = parse_expr(v);
            combos.push(KeyCombinationHashed {
                combinations: KeyCombination {
                    combination: parsed_condition,
                    action: parsed_action,
                },
                keys_hashes: key_events,
            });
        } else {
            panic!("Expected a string value for key '{}', but found {:?}", k, v)
        }
    }

    ParsedConfig {
        delay_ms: config.delay_ms,
        key_combinations: combos,
        combo_hashes: total_hashes,
    }
}

fn _read_config(path: &str) -> Result<Config, &'static str> {
    let config_str = std::fs::read_to_string(path).expect("Failed to read config.toml");
    if let Ok(config) = toml::from_str(config_str.as_str()) {
        Ok(config)
    } else {
        Err("Failed to parse config file")
    }
}

#[cfg(test)]
pub fn config_from_str(s: &str) -> ParsedConfig {
    use crate::config::_parse_config;
    use crate::config::Config;
    let raw_config: Config = toml::from_str(s).unwrap();
    _parse_config(&raw_config)
}

#[cfg(test)]
mod tests {
    use crate::key_buffer::UKey;

    use super::*;

    #[test]
    fn test_config() {
        let config = read_config().unwrap();
        let expected_key = "leftmeta + leftshift  + F23";
        assert!(
            config.main.contains_key(expected_key),
            "Config 'main' does not contain the expected key"
        );
        if let Some(toml::Value::String(v)) = config.main.get(expected_key) {
            assert_eq!(v, "leftctrl down + wait 500 + leftctrl up");
        }
    }
    #[test]
    fn test_config_parse() {
        let parsed_config = config_from_str(
            r#"
            delay_ms = 5
            [main]
            "leftmeta + leftshift  + F23" = "leftctrl down + wait 500  + leftctrl up"
            "a" = "b"
            "#,
        );

        assert_eq!(parsed_config.key_combinations.len(), 2);
        assert_eq!(parsed_config.delay_ms, Some(5));

        assert!(parsed_config.has_key(&Event {
            key: UKey::LeftShift,
            action: Action::Press
        }));
        assert!(parsed_config.has_key(&Event {
            key: UKey::LeftShift,
            action: Action::Release
        }));
        assert!(parsed_config.has_key(&Event {
            key: UKey::F23,
            action: Action::Press
        }));
        assert!(parsed_config.has_key(&Event {
            key: UKey::F23,
            action: Action::Release
        }));
        assert!(parsed_config.has_key(&Event {
            key: UKey::A,
            action: Action::Press
        }));
        assert!(parsed_config.has_key(&Event {
            key: UKey::A,
            action: Action::Release
        }));
        assert_eq!(
            parsed_config.has_key(&Event {
                key: UKey::F24,
                action: Action::Release
            }),
            false
        );
        println!("parsed {parsed_config:?}");
    }

    #[test]
    #[ignore = "Rust playground"]
    fn test_something() {
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }
        let inp = "leftctrl down + wait 500  + leftctrl up";
        let mut v = vec![1, 2, 3, 5];
        let mut ccc = || {
            println!("closure {v:?}");
            v.push(666);
        };

        fn ppp(v: &mut Vec<i32>) {
            println!("func ppp");
            v.push(1);
            println!("{v:?}");
        }
        ccc();
        ppp(&mut v);
        v.sort_by(|a, b| b.cmp(a));
        println!("{v:?}");
        // let novel = String::from("Call me Ishmael. Some years ago...");
        fn fff(sss: &str) -> ImportantExcerpt {
            let first_sentence = sss.split(' ').nth(1).unwrap();
            let i = ImportantExcerpt {
                part: first_sentence,
            };
            println!("fff: {}", i.part);
            println!("Address of i: {:p}", &i);

            i
        }

        let a = 123;
        println!("Address of a {:p}", &a);
        let i1 = inp.as_ptr() as usize;
        println!("Memory address of inp: {:p}", inp);
        let i = fff(inp);
        println!("Address of i: {:p}", &i);
        let i2 = i.part.as_ptr() as usize;
        let heap_var = Box::new(42);
        println!("Address of heap_var: {:p}", &*heap_var);
        drop(heap_var);

        println!("return : {} {i2}", i.part);
        println!("dif {}", i2 - i1);

        println!("{}", first_word(inp));
        struct Colors {}

        impl Colors {
            pub const RED: &'static str = "Red";
            pub const GREEN: &'static str = "Green";
            pub const BLUE: &'static str = "Blue";
        }
        println!("{}", Colors::RED);
    }
}
