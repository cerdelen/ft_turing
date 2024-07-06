
use core::fmt;
use std::fs::File;
use std::io::{BufReader, Read};
use crate::consts::*;
use crate::machine_description::{Action, MachineDescription};

#[derive(Debug)]
pub struct MachineTape {
    tape: Vec<char>,
    head: usize,
    blank: char,
}

#[allow(dead_code)]
pub enum HeadDirection {
    LEFT,
    RIGHT
}

impl MachineTape {
    pub fn new(input: &mut BufReader<File>, desc: &MachineDescription) -> Self {
        let mut content = String::new();
        input.read_to_string(&mut content)
            .expect(&format!("\n{}\n\n\t\t{}ICould not Read Input Tape!{}\n\n{}\n\n",
            H_BORDER, BOLD_RED_CHAR, RESET_CHAR, H_BORDER));

        // deleting new line
        if content.ends_with('\n') {
            content.pop();
        }
        // string into Vec
        let mut tape: Vec<char> = content.chars().collect();
        // Checking for illegal Character
        for (i, c) in tape.iter().enumerate() {
            if desc.get_blank() == *c {
                panic!("\n{}\n\n\t {}Blank is not allowed in Input Tape! Blank at index: {}⌲ {}{}\n\n{}\n",
                H_BORDER, BOLD_RED_CHAR, BOLD_YELLOW_CHAR, i, RESET_CHAR, H_BORDER);
            }
            if desc.part_of_alphabet(&c) == false {
                panic!("\n{}\n\n{}  Illegal Character {}[ {} ]{} not Part of the Alphabet in the Tape at Index {}⌲ {}{}\n\n{}\n",
                H_BORDER, BOLD_RED_CHAR, BOLD_YELLOW_CHAR,c, BOLD_RED_CHAR, BOLD_YELLOW_CHAR, i, RESET_CHAR, H_BORDER);
            }
        }
        if tape.len() == 0 {
            tape.push(desc.get_blank());
        }
        Self{tape, head: 0, blank: desc.get_blank()}
    }

    pub fn move_head(&mut self, direction: &Action) {
        match direction {
            Action::LEFT => {
                if self.head == 0 {
                    self.tape.insert(0, self.blank);
                }
                else {
                    self.head -= 1;
                }
            },
            Action::RIGHT => {
                if self.head == self.tape.len() - 1 {
                    self.tape.push(self.blank);
                }
                self.head = self.head + 1
            }
        };
    }

    pub fn get_full_tape(&self) -> &Vec<char> {
        &self.tape
    }

    pub fn get_read(&self) -> &char {
        self.tape.get(self.head).unwrap()
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

impl fmt::Display for MachineTape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();
        buffer.push_str(&format!("[ "));
        for c in self.tape.iter() {
            buffer.push(*c);
        }
        buffer.push_str(&format!(" ]"));
        write!(f, "{}", buffer)
    }
}
