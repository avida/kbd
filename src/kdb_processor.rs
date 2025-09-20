use std::collections::VecDeque;
use std::sync::mpsc::{self, RecvError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use timer::Guard;
use uinput::event::keyboard::Key as UKey;

extern crate chrono;
extern crate timer;

#[derive(Debug, PartialEq)]
enum Action {
    Press,
    Release,
}

#[derive(Debug, PartialEq)]
struct Event {
    key: UKey,
    action: Action,
}

struct BufferEvent {
    event: Event,
    guard: Option<Guard>,
}
impl std::fmt::Debug for BufferEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufferEvent")
            .field("event", &self.event)
            .field("guard", &self.guard.is_some())
            .finish()
    }
}

impl BufferEvent {
    fn drop(&mut self) {
        if let Some(g) = self.guard.take() {
            drop(g);
        }
    }
}

#[derive(Debug)]
struct ThreadBuffer {
    deque: Arc<Mutex<VecDeque<BufferEvent>>>,
    push_channel: mpsc::Sender<Event>,
    pop_channel: mpsc::Receiver<Event>,
}

const DELAY_MS: i64 = 500;

impl ThreadBuffer {
    pub fn push(&mut self, key: UKey, action: Action) {
        let event = Event {
            key: key,
            action: action,
        };
        self.push_channel.send(event).unwrap();
    }
    pub fn pop(&mut self) -> Option<Event> {
        match self.pop_channel.recv() {
            Ok(event) => Some(event),
            Err(_) => None,
        }
    }
}

impl ThreadBuffer {
    pub fn new() -> Self {
        let c_in = mpsc::channel::<Event>();
        let c_out = mpsc::channel::<Event>();
        let dq = Arc::new(Mutex::new(VecDeque::<BufferEvent>::new()));

        let dq_c = Arc::clone(&dq);
        thread::spawn(move || {
            let pop = &c_out.0;
            let timer = timer::Timer::new();
            loop {
                for received in &c_in.1 {
                    println!("Hi from service thread {:?}", received);
                    let dq_cc = dq_c.clone();
                    let be = BufferEvent {
                        event: received,
                        guard: Some(timer.schedule_with_delay(
                            chrono::Duration::milliseconds(DELAY_MS),
                            move || {
                                println!("Timer expired!!");
                                let mut dlq = dq_cc.lock().unwrap();
                                if let Some(e) = dlq.pop_front() {
                                    println!("be:: {:?}", e);
                                    // pop.send(e.event);
                                }
                            },
                        )),
                    };
                    let mut dlq = dq_c.lock().unwrap();
                    println!("Pushed");
                    dlq.push_back(be);
                }
            }
        });
        ThreadBuffer {
            deque: Arc::clone(&dq),
            push_channel: c_in.0,
            pop_channel: c_out.1,
        }
    }
}

pub fn process() {
    println!("Hello from processing module")
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn test_process() {
        println!("wefewfewfewfwefw >");
        process();
        assert!(true)
    }

    #[test]
    fn test_buffer() {
        let mut buf = ThreadBuffer::new();
        buf.push(UKey::A, Action::Press);
        buf.push(UKey::B, Action::Release);

        let t = thread::spawn(move || {
            thread::sleep(Duration::from_secs(1));
            println!("Hi from thread");
        });
        println!("{:?}", buf);
        t.join().unwrap();

        println!("Wait {:?}", buf);
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
                key: UKey::A,
                action: Action::Release
            })
        );
    }

    #[test]
    fn test_timer() {
        let timer = timer::Timer::new();

        let mut guards = Vec::new();
        let cntr = Arc::new(Mutex::new(0));
        for _ in 0..10_000 {
            let counter = Arc::clone(&cntr);
            let guard = timer.schedule_with_delay(chrono::Duration::milliseconds(1), move || {
                // println!("Timer dingg!! after {} ms", 10);
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
}
