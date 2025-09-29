use std::hash::{DefaultHasher, Hash, Hasher};

use super::KeyCombinationHashed;
use super::parser::{Expr, Expressions};
use crate::key_buffer::{Action, Event, KeyDeque};

fn process_config() {}

pub fn get_action<'a>(
    deq: &KeyDeque,
    combinations: &'a Vec<KeyCombinationHashed>,
) -> Option<&'a Expressions> {
    let mut key_hashes = Vec::<u64>::with_capacity(deq.len());
    for event in deq.iter() {
        let mut hasher = DefaultHasher::new();
        event.event.hash(&mut hasher);
        let hash = hasher.finish();
        key_hashes.push(hash);
    }

    // Macro that returns true if all hashes are in combo
    macro_rules! all_hashes_in_combo {
        ($combo:expr, $key_hashes:expr) => {
            if $key_hashes.len() < $combo.keys_hashes.len() {
                false
            } else {
                $combo.keys_hashes.iter().all(|h| $key_hashes.contains(h))
            }
        };
    }

    for c in combinations {
        if all_hashes_in_combo!(c, key_hashes) {
            return Some(&c.combinations.action);
        }
    }
    return None;
}

pub fn action_to_events(action: &Expressions) -> Vec<(i64, Event)> {
    let mut current_delay: i64 = 0;
    let mut result = Vec::<(i64, Event)>::with_capacity(action.len());
    for expr in action {
        match expr {
            Expr::Key(key_expr) => {
                if let Some(action) = key_expr.action {
                    result.push((
                        current_delay,
                        Event {
                            action: action,
                            key: key_expr.key,
                        },
                    ));
                } else {
                    result.push((
                        current_delay,
                        Event {
                            action: Action::Press,
                            key: key_expr.key,
                        },
                    ));
                    result.push((
                        current_delay,
                        Event {
                            action: Action::Release,
                            key: key_expr.key,
                        },
                    ));
                }
            }
            Expr::Wait(expr) => {
                current_delay += expr.milliseconds as i64;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::config::parser::KeyExpr;
    use crate::key_buffer::{Action, BufferEvent, Event, KeyDeque, UKey};

    use super::super::config_from_str;
    use super::*;

    macro_rules! events_deque {
            ( $( ( $key:expr, $action:expr ) ),* $(,)? ) => {{
            let mut deq = KeyDeque::new();
            $(
                deq.push_back(BufferEvent {
                event: Event {
                    key: $key,
                    action: $action,
                },
                guard: None,
                });
            )*
            deq
            }};
        }

    #[test]
    fn test_config_processor() {
        let config = config_from_str(
            r#"
            [main]
            "leftmeta + leftshift + F23" = "leftctrl down + wait 500 + leftctrl up"
            "a" = "b"
            "n down" = "b + c"
            "#,
        );
        let deq = events_deque!(
            (UKey::LeftMeta, Action::Press),
            (UKey::LeftShift, Action::Press),
            (UKey::F23, Action::Press),
            (UKey::LeftMeta, Action::Release),
            (UKey::LeftShift, Action::Release),
            (UKey::F23, Action::Release),
        );
        let action = get_action(&deq, &config.key_combinations);
        assert_eq!(action.unwrap().len(), 3);

        let deq = events_deque!((UKey::LeftMeta, Action::Press),);
        let action = get_action(&deq, &config.key_combinations);
        assert!(action.is_none());

        let deq = events_deque!((UKey::A, Action::Press), (UKey::A, Action::Release),);
        let action = get_action(&deq, &config.key_combinations);
        assert_eq!(action.unwrap().len(), 1);

        let deq = events_deque!((UKey::N, Action::Press));
        let action = get_action(&deq, &config.key_combinations);
        assert_eq!(action.unwrap().len(), 2);

        // Some extra keys are present
        let deq = events_deque!((UKey::N, Action::Press), (UKey::A, Action::Release),);
        let action = get_action(&deq, &config.key_combinations);
        assert_eq!(action.unwrap().len(), 2);

        // Some extra keys are present
        let deq = events_deque!((UKey::C, Action::Press), (UKey::A, Action::Release),);
        let action = get_action(&deq, &config.key_combinations);
        assert!(action.is_none());
    }
    #[test]
    fn test_action() {
        let combo = vec![Expr::Key(KeyExpr {
            key: UKey::A,
            action: Some(Action::Press),
        })];
        assert_eq!(
            action_to_events(&combo),
            vec![(
                0,
                Event {
                    key: UKey::A,
                    action: Action::Press
                }
            )]
        );
        let combo = vec![Expr::Key(KeyExpr {
            key: UKey::A,
            action: None,
        })];
        assert_eq!(
            action_to_events(&combo),
            vec![
                (
                    0,
                    Event {
                        key: UKey::A,
                        action: Action::Press
                    }
                ),
                (
                    0,
                    Event {
                        key: UKey::A,
                        action: Action::Release
                    }
                ),
            ]
        );

        let combo = vec![Expr::Key(KeyExpr {
            key: UKey::LeftControl,
            action: Some(Action::Press),
        }),
        Expr::Wait(crate::config::parser::WaitExpr { milliseconds: 500 }),
        Expr::Key(KeyExpr {
            key: UKey::LeftControl,
            action: Some(Action::Release),
        }),
        ];
        assert_eq!(
            action_to_events(&combo),
            vec![
                (
                    0,
                    Event {
                        key: UKey::LeftControl,
                        action: Action::Press
                    }
                ),
                (
                    500,
                    Event {
                        key: UKey::LeftControl,
                        action: Action::Release
                    }
                ),
            ]
        );
    }
}
