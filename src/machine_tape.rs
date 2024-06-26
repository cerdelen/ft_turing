use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

use crate::machine_description::{Action, MachineDescription};

use std::fmt;

#[derive(Debug)]
pub enum RunningOfTapeErr {
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
    tape: Vec<char>,
    head: usize,
}

pub enum HeadDirection {
    LEFT,
    RIGHT
}

impl MachineTape {
    pub fn new(input: &mut BufReader<File>, desc: &MachineDescription) -> Self {
        let mut content = String::new();
        input.read_to_string(&mut content).expect("Could not Read Input Tape");
        // deleting new line
        if content.ends_with('\n') {
            content.pop();
        }
        // string into Vec
        let tape: Vec<char> = content.chars().collect();
        // Checking for illegal Character
        for (i, c) in tape.iter().enumerate() {
            if desc.part_of_alphabet(&c) == false {
                panic!("Illegal Character ({}) not Part of the Alphabet in the Tape at Index {}\n", c, i);
            }
        }
        Self{tape, head: 0}
    }

    pub fn move_head(&mut self, direction: &Action) -> Result<(), RunningOfTapeErr> {
        match direction {
            Action::LEFT => self.head = self.head.checked_sub(1).ok_or(RunningOfTapeErr::LEFT)?,
            Action::RIGHT => {
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

    pub fn get_full_tape(&self) -> &Vec<char> {
        &self.tape
    }

    pub fn get_read(&self) -> &char {
        self.tape.get(self.head).unwrap()
        // self.tape.chars().nth(self.head).unwrap()
    }

    pub fn perform_write(&mut self, value: &char) {
        let char_to_change = match self.tape.get_mut(self.head) {
            Some(c) => c,
            None => {
                panic!("{}", &format!("Head Position is not on Tape!\nHead: {}, Tape Lengt: {}", self.head, self.tape.len()));
            },
        };
        *char_to_change = *value;
    }

    pub fn get_head_pos(&self) -> usize {
        self.head
    }
}
