#![allow(dead_code)]

use crate::debug_println;
use std::collections::VecDeque;
use std::error::Error;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use timer::Guard;
use uinput::event::keyboard::Key as UKey;

extern crate chrono;
extern crate timer;

const DELAY_MS: i64 = 5;
const KEY_CAPASITY: usize = 10;

#[derive(Debug, PartialEq)]
pub enum Action {
    Press,
    Release,
}

#[derive(Debug, PartialEq)]
pub struct Event {
    pub key: UKey,
    pub action: Action,
}

struct BufferEvent {
    event: Event,
    guard: Option<Guard>,
}

static pattern: [Event; 6] = [
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
    Event {
        key: UKey::LeftMeta,
        action: Action::Release,
    },
    Event {
        key: UKey::LeftShift,
        action: Action::Release,
    },
    Event {
        key: UKey::F23,
        action: Action::Release,
    },
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

type SyncReceiver<T> = Arc<Mutex<mpsc::Receiver<T>>>;
type KeyDeque = VecDeque<BufferEvent>;

#[derive(Debug)]
pub struct KeyBuffer {
    deque: Arc<Mutex<KeyDeque>>,
    push_channel: mpsc::Sender<Event>,
    pop_channel: SyncReceiver<Event>,
}

impl KeyBuffer {
    pub fn push(&self, key: UKey, action: Action) {
        let event = Event {
            key: key,
            action: action,
        };
        if DELAY_MS == 0 {
            self.push_channel.send(event).unwrap();
        } else {
            self.push_channel.send(event).unwrap();
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

    fn _drop(deque: &mut KeyDeque) {
        for el in deque.iter_mut() {
            el.cancel();
        }
        deque.clear();
    }

    fn _schedule(deque: &mut KeyDeque, event: Event) {

    }
}

impl KeyBuffer {
    fn _gotcha(deq: &KeyDeque) -> bool {
        if pattern.len() != deq.len() {
            return false;
        }
        for (buf_event, pat_event) in deq.iter().zip(pattern.iter()) {
            if buf_event.event != *pat_event {
                return false;
            }
        }
        true
    }
    fn _start_listen(& mut self) {

    }

    pub fn new() -> Result<Self, Box<dyn Error>> {
        let c_in = mpsc::channel::<Event>();
        let c_out = mpsc::channel::<Event>();
        let dq: Arc<Mutex<VecDeque<BufferEvent>>> = Arc::new(Mutex::new(
            VecDeque::<BufferEvent>::with_capacity(KEY_CAPASITY),
        ));

        let dq_c = dq.clone();
        thread::spawn(move || {
            let pop = &c_out.0;
            let timer = timer::Timer::new();
            loop {
                for received in &c_in.1 {
                    let dq_cc = dq_c.clone();
                    let pop_clone = pop.clone();
                    let be = BufferEvent {
                        event: received,
                        guard: Some(timer.schedule_with_delay(
                            chrono::Duration::milliseconds(DELAY_MS),
                            move || {
                                let mut dlq = dq_cc.lock().unwrap();
                                if let Some(e) = dlq.pop_front() {
                                    pop_clone.send(e.event).unwrap();
                                }
                            },
                        )),
                    };
                    let mut dlq = dq_c.lock().unwrap();
                    dlq.push_back(be);
                    debug_println!("Buffer size after push: {}", dlq.len());
                    if KeyBuffer::_gotcha(&dlq) {
                        KeyBuffer::_drop(&mut dlq);
                        debug_println!("GOIDAAAAAAAAAAAAAA!!");
                        let pop_clone = pop.clone();
                        pop_clone.send(Event { key: UKey::A, action: Action::Press }).unwrap();
                        let mut  g= Some(timer.schedule_with_delay(chrono::Duration::milliseconds(200), move||{
                            debug_println!("GOIDa 222!!");
                        }));

                    }
                }
            }
        });
        Ok(KeyBuffer {
            deque: dq,
            push_channel: c_in.0,
            pop_channel: Arc::new(Mutex::new(c_out.1)),
        })
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
        let mut buf = KeyBuffer::new().unwrap();
        buf.push(UKey::A, Action::Press);
        buf.push(UKey::B, Action::Release);
        buf.push(UKey::C, Action::Release);
        thread::sleep(Duration::from_millis(1));
        let mut dlq = buf.deque.lock().unwrap();
        KeyBuffer::_drop(&mut dlq);
        thread::sleep(Duration::from_millis(300));
        assert_eq!(buf.try_pop(), None);
        assert_eq!(dlq.len(), 0);
    }

    #[test]
    fn test_timer_simple() {
        let timer = timer::Timer::new();
        println!("schedule");
        let mut g = timer.schedule_with_delay(chrono::Duration::milliseconds(100), move || {
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
