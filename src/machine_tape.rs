use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};


use std::fmt;

#[derive(Debug)]
enum RunningOfTapeErr {
    LEFT,
    RIGHT,
}

impl fmt::Display for RunningOfTapeErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunningOfTapeErr::LEFT => write!(f, "Ran off the Tape to the Left"),
            RunningOfTapeErr::RIGHT => write!(f, "Ran off the Tape to the Right"),
        }
    }
}

impl std::error::Error for RunningOfTapeErr {}





#[derive(Debug)]
pub struct MachineTape {
    tape: String,
    head: usize,
}

pub enum HeadDirection {
    LEFT,
    RIGHT
}

impl MachineTape {
    pub fn new(input: &mut BufReader<File>) -> Self {
        let mut content = String::new();
        input.read_to_string(&mut content).expect("Could not Read Input Tape");
        Self{tape: content, head: 0}
    }

    pub fn move_head(&mut self, direction: HeadDirection) -> Result<(), RunningOfTapeErr> {
        match direction {
            HeadDirection::LEFT => self.head = self.head.checked_sub(1).ok_or(RunningOfTapeErr::LEFT)?,
            HeadDirection::RIGHT => {
                if self.head == self.tape.len() {
                    return Err(RunningOfTapeErr::RIGHT);
                }
                else {
                    self.head = self.head + 1
                }
            }
        };
        Ok(())
    }

    fn current_character(&self) -> Option<char> {
        self.tape.chars().nth(self.head)
    }
}
