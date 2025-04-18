use std::time::SystemTime;

use rdev::Event;

use enigo::*;
use std::thread;
use std::time::Duration;

const WAKE_UP_EVALUATION_TIME: u128 = 1000;

#[derive(Debug)]
enum State {
    Sleeping,
    Expecting,
    Listening,
    Evaluating,
}

pub struct Assistant {
    state: State,
    pressed_time: Option<SystemTime>,
    command: String,
    wake_char: String,
}

impl Assistant {
    pub fn new(wake_char: String) -> Self {
        Assistant {
            state: State::Sleeping,
            pressed_time: None,
            command: "".to_owned(),
            wake_char,
        }
    }

    pub fn callback(&mut self, event: Event) {
        match self.state {
            State::Sleeping => match event.name {
                Some(string) => self.evaluate_word(string, event.time),
                None => (),
            },
            State::Expecting => match event.name {
                Some(string) => self.compute_word_diff(string, event.time),
                None => (),
            },
            State::Listening => match event.name {
                Some(string) => self.listening_command(string),
                None => (),
            },
            _ => {}
        }
    }

    fn reset(&mut self) {
        println!("reset");
        self.state = State::Sleeping;
        self.pressed_time = None;
        self.command = "".to_owned();
    }

    fn evaluate(&mut self) {
        println!("evaluating {:?}", self.command);
        send_text_to_context(&self.command);
        self.reset();
    }

    fn evaluate_word(&mut self, keypress: String, pressed_time: SystemTime) {
        match keypress {
            val if val == self.wake_char.to_owned() => {
                println!("pressed {:?} at {:?}", self.wake_char, pressed_time);
                self.pressed_time = Some(pressed_time);
                self.state = State::Expecting;
            }
            _ => {}
        }
    }

    fn compute_word_diff(&mut self, keypress: String, pressed_time: SystemTime) {
        match keypress {
            val if val == self.wake_char.to_owned() => {
                println!(
                    "pressed {:?} in compute_word_diff at {:?}",
                    self.wake_char, pressed_time
                );
                if let Some(last_pressed_time) = self.pressed_time {
                    match pressed_time.duration_since(last_pressed_time) {
                        Ok(duration) => {
                            println!("Time elapsed: {} ms", duration.as_millis());

                            if duration.as_millis() <= WAKE_UP_EVALUATION_TIME {
                                self.state = State::Listening;
                            } else {
                                self.reset();
                            }
                        }
                        Err(e) => {
                            self.reset();
                            println!("Error calculating time difference: {:?}", e);
                        }
                    }
                }
                println!("pressed {:?} at {:?}", self.wake_char, pressed_time);
                self.pressed_time = Some(pressed_time);
            }
            _ => {
                self.reset();
            }
        }
    }

    fn listening_command(&mut self, keypress: String) {
        println!("pressed {:?}", keypress);
        match keypress {
            val if val == "\r".to_owned() => {
                self.state = State::Evaluating;
                self.evaluate()
            }
            _ => {
                self.command += &keypress;
            }
        }
    }
}

fn send_text_to_context(text: &str) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    // Wait briefly to ensure the target window is focused
    thread::sleep(Duration::from_millis(50));
    let _ = enigo.text(&("\r".to_string() + text));
}
