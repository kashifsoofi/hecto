//importing in execute! macro
// #[macro_use]
// extern crate crossterm;

use std::io::{self, Read};
use crossterm::{
    event::{
        read, Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind, KeyEventState,
    },
    // style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

fn to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        disable_raw_mode().expect("Unable to disable raw mode");
    }
}

fn main() {
    let _cleanup = Cleanup;
    enable_raw_mode().expect("Failed to enable raw mode");
    loop {
        if let Event::Key(event) = read().unwrap() {
            match event {
                KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                } => break,
                KeyEvent {
                    code: key_code @ KeyCode::Char(..),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                } => {
                    match key_code {
                        KeyCode::Char(c) => {
                            println!("{}\r", c);
                        },
                        _ => {},
                    }
                },
                _ => {}
            }
        }
    }
}
