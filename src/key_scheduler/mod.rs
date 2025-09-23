use std::collections::HashMap;
use std::error::Error;

use crate::key_buffer::{Event, SafeSender};
use id_generator::{IdGenerator, MAX_ID};
use std::sync::{Arc, Mutex};

mod id_generator;
pub struct KeyScheduler {
    timer: timer::Timer,
    sender: SafeSender,
    guards: Arc<Mutex<HashMap<u8, timer::Guard>>>,
    id_generator: Arc<Mutex<IdGenerator>>,
}

impl KeyScheduler {
    pub fn new(sender: SafeSender) -> Result<Self, Box<dyn Error>> {
        Ok(KeyScheduler {
            timer: timer::Timer::new(),
            sender: sender,
            guards: Arc::new(Mutex::new(HashMap::<u8, timer::Guard>::with_capacity(
                MAX_ID as usize,
            ))),
            id_generator: Arc::new(Mutex::new(IdGenerator::new())),
        })
    }

    pub fn schedule(&mut self, event: Event, delay_ms: i64) -> Result<(), Box<dyn Error>> {
        {
            let guards = self.guards.lock().unwrap();
            if guards.len() >= MAX_ID as usize {
                // Too many events scheduled
                return Err("Maximum number of scheduled events reached".into());
            }
        }

        let s = self.sender.clone();
        let guards = self.guards.clone();
        let id = self.id_generator.lock().unwrap().next().unwrap();
        let g =
            self.timer
                .schedule_with_delay(chrono::Duration::milliseconds(delay_ms), move || {
                    s.lock().unwrap().send(event.clone()).unwrap();
                    guards.lock().unwrap().remove(&id);
                });
        self.guards.lock().unwrap().insert(id, g);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key_buffer::{Action, Event, UKey};
    use std::sync::mpsc::channel;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_scheduler() {
        let (tx, rx) = channel::<Event>();
        let tx = Arc::new(Mutex::new(tx));

        let mut ks = KeyScheduler::new(tx).unwrap();
        ks.schedule(
            Event {
                key: UKey::A,
                action: Action::Release,
            },
            300,
        ).unwrap();
        ks.schedule(
            Event {
                key: UKey::A,
                action: Action::Press,
            },
            30,
        ).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(400));

        let received = rx.try_recv().unwrap();
        assert_eq!(
            received,
            Event {
                key: UKey::A,
                action: Action::Press,
            }
        );
        let received = rx.try_recv().unwrap();
        assert_eq!(
            received,
            Event {
                key: UKey::A,
                action: Action::Release,
            }
        );
    }
    #[test]
    fn test_scheduler_zero_delay() {
        let (tx, rx) = channel::<Event>();
        let tx = Arc::new(Mutex::new(tx));

        let mut ks = KeyScheduler::new(tx).unwrap();
        ks.schedule(
            Event {
                key: UKey::B,
                action: Action::Press,
            },
            0,
        ).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(10));
        let received = rx.try_recv().unwrap();
        assert_eq!(
            received,
            Event {
                key: UKey::B,
                action: Action::Press,
            }
        );
    }

    #[test]
    fn test_scheduler_negative_delay() {
        let (tx, rx) = channel::<Event>();
        let tx = Arc::new(Mutex::new(tx));

        let mut ks = KeyScheduler::new(tx).unwrap();
        ks.schedule(
            Event {
                key: UKey::C,
                action: Action::Release,
            },
            -100,
        ).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(10));
        let received = rx.try_recv().unwrap();
        assert_eq!(
            received,
            Event {
                key: UKey::C,
                action: Action::Release,
            }
        );
    }

    #[test]
    fn test_scheduler_multiple_events_same_key() {
        let (tx, rx) = channel::<Event>();
        let tx = Arc::new(Mutex::new(tx));

        let mut ks = KeyScheduler::new(tx).unwrap();
        for i in 0..5 {
            ks.schedule(
                Event {
                    key: UKey::D,
                    action: if i % 2 == 0 {
                        Action::Press
                    } else {
                        Action::Release
                    },
                },
                10 * i,
            ).unwrap();
        }

        std::thread::sleep(std::time::Duration::from_millis(60));
        let mut results = vec![];
        while let Ok(event) = rx.try_recv() {
            results.push(event);
        }
        assert_eq!(results.len(), 5);
        assert_eq!(
            results[0],
            Event {
                key: UKey::D,
                action: Action::Press,
            }
        );
        assert_eq!(
            results[1],
            Event {
                key: UKey::D,
                action: Action::Release,
            }
        );
    }

    #[test]
    fn test_no_event_sent_if_not_scheduled() {
        let (_tx, rx) = channel::<Event>();
        std::thread::sleep(std::time::Duration::from_millis(50));
        assert!(rx.try_recv().is_err());
    }

    #[test]
    fn test_schedule_many_events() {
        let (tx, rx) = channel::<Event>();
        let tx = Arc::new(Mutex::new(tx));
        let mut ks = KeyScheduler::new(tx).unwrap();
        const EVENTS: i64 = 200;

        for i in 0..EVENTS {
            ks.schedule(
                Event {
                    key: UKey::A,
                    action: if i % 2 == 0 {
                        Action::Press
                    } else {
                        Action::Release
                    },
                },
                i,
            )
            .unwrap();
        }

        std::thread::sleep(std::time::Duration::from_millis(300));
        let mut count = 0;
        while let Ok(_) = rx.try_recv() {
            count += 1;
        }
        assert_eq!(count, EVENTS);
    }
    #[test]
    fn test_schedule_too_many_events() {
        let (tx, rx) = channel::<Event>();
        let tx = Arc::new(Mutex::new(tx));
        let mut ks = KeyScheduler::new(tx).unwrap();
        for i in 0..MAX_ID {
            ks.schedule(
                Event {
                    key: UKey::A,
                    action: if i % 2 == 0 {
                        Action::Press
                    } else {
                        Action::Release
                    },
                },
                50,
            )
            .unwrap();
        }
        let result = ks.schedule(
            Event {
                key: UKey::A,
                action: Action::Press,
            },
            0,
        );
        assert!(result.is_err());

        std::thread::sleep(std::time::Duration::from_millis(100));
        let mut count = 0;
        while let Ok(_) = rx.try_recv() {
            count += 1;
        }
        assert_eq!(count, MAX_ID);
    }
}
