#![allow(dead_code)]

use crate::key_scheduler::KeyScheduler;
use crate::config::{ParsedConfig, get_action};
use crate::debug_println;
use std::collections::VecDeque;
use std::collections::hash_map::DefaultHasher;
use std::error::Error;
use std::hash::{Hash, Hasher};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use timer::Guard;
use uinput::event::Code;
pub use uinput::event::keyboard::Key as UKey;

extern crate chrono;
extern crate timer;

const DELAY_MS: i64 = 3;
const KEY_CAPASITY: usize = 10;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum Action {
    Press,
    Release,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Event {
    pub key: UKey,
    pub action: Action,
}

impl Event {
    pub fn get_u64_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}
impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.code().cmp(&other.key.code())
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct BufferEvent {
    pub event: Event,
    pub guard: Option<Guard>,
}

static PATTERN: [Event; 3] = [
    Event {
        key: UKey::LeftMeta,
        action: Action::Press,
    },
    Event {
        key: UKey::LeftShift,
        action: Action::Press,
    },
    Event {
        key: UKey::F23,
        action: Action::Press,
    },
    // Event {
    //     key: UKey::LeftMeta,
    //     action: Action::Release,
    // },
    // Event {
    //     key: UKey::LeftShift,
    //     action: Action::Release,
    // },
    // Event {
    //     key: UKey::F23,
    //     action: Action::Release,
    // },
];

impl std::fmt::Debug for BufferEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferEvent")
            .field("event", &self.event)
            .field("guard", &self.guard.is_some())
            .finish()
    }
}

impl BufferEvent {
    fn cancel(&mut self) {
        if let Some(g) = self.guard.take() {
            drop(g);
            self.guard = None;
        }
    }
}

type SafeReceiver = Arc<Mutex<mpsc::Receiver<Event>>>;
pub type SafeSender = Arc<Mutex<mpsc::Sender<Event>>>;
pub type KeyDeque = VecDeque<BufferEvent>;

pub struct KeyBuffer<'a> {
    deque: Arc<Mutex<KeyDeque>>,
    push_channel: SafeSender,
    _push_channel_r: SafeReceiver,

    pop_channel: SafeReceiver,
    _pop_channel_s: SafeSender,
    timer: timer::Timer,
    key_scheduler: Arc<Mutex<KeyScheduler>>,
    config: &'a ParsedConfig,
}

impl<'a> KeyBuffer<'a> {
    pub fn push(&self, key: UKey, action: Action) {
        let event = Event {
            key: key,
            action: action,
        };
        if DELAY_MS == 0 {
            self.push_channel.lock().unwrap().send(event).unwrap();
        } else {
            self.push_channel.lock().unwrap().send(event).unwrap();
        }
    }
    pub fn pop(&self) -> Option<Event> {
        let c = self.pop_channel.clone();
        let locked_c = c.lock().unwrap();
        match locked_c.recv() {
            Ok(event) => Some(event),
            Err(_) => None,
        }
    }

    pub fn try_pop(&self) -> Option<Event> {
        let c = self.pop_channel.clone();
        let locked_c = c.lock().unwrap();
        match locked_c.try_recv() {
            Ok(event) => Some(event),
            Err(_) => None,
        }
    }

    fn _drop(self: Arc<Self>) {
        let mut deque = self.deque.lock().unwrap();
        for el in deque.iter_mut() {
            el.cancel();
        }
        deque.clear();
    }
}

impl <'a>KeyBuffer<'a> {
    fn _gotcha(self: Arc<Self>) -> bool {
        let deq = self.deque.lock().unwrap();
        if PATTERN.len() != deq.len() {
            return false;
        }
        for (buf_event, pat_event) in deq.iter().zip(PATTERN.iter()) {
            if buf_event.event != *pat_event {
                return false;
            }
        }
        true
    }

    fn _schedule_event(self: Arc<Self>, event: Event, delay: i64) {
        debug_println!("Scheduled {:?} {}", event, delay);
        let self_c = self.clone();
        let be = BufferEvent {
            event: event,
            guard: Some(self.timer.schedule_with_delay(
                chrono::Duration::milliseconds(delay),
                move || {
                    let mut dlq = self_c.deque.lock().unwrap();

                    if let Some(e) = dlq.pop_front() {
                        self_c._pop_channel_s.lock().unwrap().send(e.event).unwrap();
                    }
                },
            )),
        };
        let mut dlq = self.deque.lock().unwrap();
        dlq.push_back(be);
    }

    fn _start_listen(key_buffer: Arc<Self>) {
        thread::spawn(move || {
            let kb = key_buffer.clone();
            loop {
                if let Ok(received) = kb._push_channel_r.lock().unwrap().recv() {
                    kb.clone()._schedule_event(received, DELAY_MS);
                    debug_println!("Buffer size after push: {}", kb.deque.lock().unwrap().len());
                    if kb.clone()._gotcha() {
                        kb.clone()._drop();
                        {
                            let mut locked_scheduler = kb.key_scheduler.lock().unwrap();
                           macro_rules! try_schedule {
                                ($scheduler:expr, $event:expr, $delay:expr) => {
                                    if let Err(e) = $scheduler.schedule($event, $delay) {
                                        eprintln!("Error scheduling event: {}", e);
                                    }
                                };
                            }
                            try_schedule!(
                                locked_scheduler,
                                Event {
                                    key: UKey::RightControl,
                                    action: Action::Press,
                                },
                                0
                            );
                            try_schedule!(
                                locked_scheduler,
                                Event {
                                    key: UKey::RightControl,
                                    action: Action::Release,
                                },
                                500
                            );
                        }
                    }
                }
            }
        });
    }

    pub fn new(app_config: &'a ParsedConfig) -> Result<Arc<Self>, Box<dyn Error>> {
        let c_in = mpsc::channel::<Event>();
        let c_out = mpsc::channel::<Event>();
        macro_rules! make_recv {
            ($arg:expr) => {
                Arc::new(Mutex::new($arg))
            };
        }

        let push_channel_ptr = make_recv!(c_in.0);
        let kb = Arc::new(KeyBuffer {
            deque: make_recv!(VecDeque::<BufferEvent>::with_capacity(KEY_CAPASITY)),
            push_channel: push_channel_ptr.clone(),
            _push_channel_r: make_recv!(c_in.1),

            pop_channel: make_recv!(c_out.1),
            _pop_channel_s: make_recv!(c_out.0),
            timer: timer::Timer::new(),
            key_scheduler: make_recv!(KeyScheduler::new(push_channel_ptr.clone()).unwrap()),
            config:&app_config,
        });
        KeyBuffer::_start_listen(kb.clone());
        Ok(kb.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::time::Duration;

    #[test]
    fn test_buffer() {
        let buf = KeyBuffer::new().unwrap();
        buf.push(UKey::A, Action::Press);
        buf.push(UKey::B, Action::Release);
        buf.push(UKey::C, Action::Release);

        assert_eq!(
            buf.pop(),
            Some(Event {
                key: UKey::A,
                action: Action::Press
            })
        );
        assert_eq!(
            buf.pop(),
            Some(Event {
                key: UKey::B,
                action: Action::Release
            })
        );
        assert_eq!(
            buf.pop(),
            Some(Event {
                key: UKey::C,
                action: Action::Release
            })
        );
        assert_eq!(buf.try_pop(), None);
    }

    #[test]
    fn test_buffer_drop() {
        let buf = KeyBuffer::new().unwrap();
        buf.push(UKey::A, Action::Press);
        buf.push(UKey::B, Action::Release);
        buf.push(UKey::C, Action::Release);
        thread::sleep(Duration::from_millis(1));
        buf.clone()._drop();
        thread::sleep(Duration::from_millis(300));
        assert_eq!(buf.try_pop(), None);
        assert_eq!(buf.deque.lock().unwrap().len(), 0);
    }

    #[test]
    fn test_timer_simple() {
        let timer = timer::Timer::new();
        println!("schedule");
        let _g = timer.schedule_with_delay(chrono::Duration::milliseconds(100), move || {
            println!("WEFWEFWEF");
        });

        thread::sleep(Duration::from_secs(1));
    }

    #[test]
    fn test_timer() {
        let timer = timer::Timer::new();

        let mut guards = Vec::new();
        let cntr = Arc::new(Mutex::new(0));
        for _ in 0..10_000 {
            let counter = Arc::clone(&cntr);
            let guard = timer.schedule_with_delay(chrono::Duration::milliseconds(1), move || {
                println!("Timer dingg!! after {} ms", 10);
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            guards.push(guard);
        }
        for g in guards {
            drop(g);
        }
        println!("Events {}", cntr.lock().unwrap());
        thread::sleep(Duration::from_secs(1));
    }
    macro_rules! dashes {
        () => {
            let dashes = "-".repeat(50);
            println!("{}", dashes);
        };
    }

    #[test]
    fn test_deque() {
        dashes!();
        let mut v = VecDeque::<BufferEvent>::new();
        let a = [
            Event {
                key: UKey::A,
                action: Action::Press,
            },
            Event {
                key: UKey::A,
                action: Action::Press,
            },
            Event {
                key: UKey::A,
                action: Action::Release,
            },
        ];
        v.push_back(BufferEvent {
            event: Event {
                key: UKey::A,
                action: Action::Press,
            },
            guard: None,
        });
        v.push_back(BufferEvent {
            event: Event {
                key: UKey::A,
                action: Action::Press,
            },
            guard: None,
        });
        v.push_back(BufferEvent {
            event: Event {
                key: UKey::A,
                action: Action::Release,
            },
            guard: None,
        });
        println!("{:?}", v);

        assert_eq!(a.len(), v.len());
        for (i, elem) in v.iter().enumerate() {
            println!("{:?} {:?}, {:?}", i, elem, a);
            assert_eq!(elem.event, a[i]);
        }
        dashes!();
    }
}
