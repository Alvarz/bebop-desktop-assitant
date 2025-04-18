mod context_handler;
use context_handler::{get_selected_text, send_text_to_context};
use rdev::Event;
use std::time::SystemTime;

use enigo::{
    Enigo, Settings,
};

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
        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        if let Some(text) = get_selected_text(&mut enigo) {
            self.command += &(" ".to_string() + &text);
        }

        // send_text_to_context(&self.command, &mut enigo);
        send_text_to_context("response text", &mut enigo);
        self.reset();
    }

    fn evaluate_word(&mut self, keypress: String, pressed_time: SystemTime) {
        match keypress {
            val if val == self.wake_char.to_owned() => {
                self.pressed_time = Some(pressed_time);
                self.state = State::Expecting;
            }
            _ => {}
        }
    }

    fn compute_word_diff(&mut self, keypress: String, pressed_time: SystemTime) {
        match keypress {
            val if val == self.wake_char.to_owned() => {
                if let Some(last_pressed_time) = self.pressed_time {
                    match pressed_time.duration_since(last_pressed_time) {
                        Ok(duration) => {
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
                self.pressed_time = Some(pressed_time);
            }
            _ => {
                self.reset();
            }
        }
    }

    fn listening_command(&mut self, keypress: String) {
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
