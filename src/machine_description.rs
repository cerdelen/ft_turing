use serde::Deserializer;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use crate::consts::*;

pub enum DescriptionErrors {
    NoTransitionsForState,
    NoTransitionForReadInState,
}

#[derive(Debug)]
pub enum Action {
    RIGHT,
    LEFT,
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.as_str() {
            "LEFT" => Ok(Action::LEFT),
            "RIGHT" => Ok(Action::RIGHT),
            _ => Err(serde::de::Error::custom("Invalid value for Direction")),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Transition {
    read: char,
    pub to_state: String,
    pub write: char,
    pub action: Action,
}

#[derive(Deserialize, Debug)]
pub struct MachineDescription {
    name: String,
    alphabet: Vec<char>,
    blank: char,
    states: Vec<String>,
    initial: String,
    finals: Vec<String>,
    transitions: HashMap<String, Vec<Transition>>,
}

impl MachineDescription {
    pub fn new(input: BufReader<File>) -> (Self, String) {
        let ret: Self = serde_json::from_reader(input).expect("Invalid Json");
        let initial = ret.initial.clone();
        (ret, initial)
    }

    // get all Transitions for given State
    fn get_transition_vec(&self, state: &String) -> Option<&Vec<Transition>> {
        self.transitions.get(state)
    }

    // get Transition for given State AND current read
    pub fn get_transition(
        &self,
        state: &String,
        current_read: &char,
    ) -> Result<&Transition, DescriptionErrors> {
        let tansitions = self
            .get_transition_vec(state)
            .ok_or(DescriptionErrors::NoTransitionsForState)?;
        let transition = tansitions.iter().find(|&entry| entry.read == *current_read);
        let ret = transition.ok_or(DescriptionErrors::NoTransitionForReadInState)?;
        Ok(ret)
    }

    pub fn check_for_end(&self, state: &String) -> bool {
        self.finals.contains(state)
    }

    pub fn part_of_alphabet(&self, c: &char) -> bool {
        self.alphabet.contains(c)
    }

    pub fn get_blank(&self) -> char {
        self.blank
    }
}

impl fmt::Display for Transition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\'{}{}{}\' -> (write: \'{}{}{}\', action: {}{:?}{}, change_to: {}{}{})",
        BOLD_PINK_CHAR, self.read, RESET_CHAR, CYAN_CHAR, self.write, RESET_CHAR, CYAN_CHAR,
        self.action, RESET_CHAR, CYAN_CHAR, self.to_state, RESET_CHAR)?;
        Ok(())
    }
}
impl fmt::Display for MachineDescription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n\n{}", H_BORDER)?;
        writeln!(f, "{}", V_BORDER)?;
        writeln!(f, "* {}{}\t\t\t\t{:<46}{} *", CYAN_CHAR, BOLD_CHAR, self.name, RESET_CHAR)?;
        writeln!(f, "{}", V_BORDER)?;
        writeln!(f, "{}\n\n", H_BORDER)?;
        writeln!(f, "{}Alphabet :{} {}{:?}{}\n", BOLD_PINK_CHAR, RESET_CHAR, GREEN_CHAR, self.alphabet, RESET_CHAR)?;
        writeln!(f, "{}States   :{} {}{:?}{}\n", BOLD_PINK_CHAR, RESET_CHAR, GREEN_CHAR, self.states, RESET_CHAR)?;
        writeln!(f, "{}Initial  :{} {}{:?}{}\n", BOLD_PINK_CHAR, RESET_CHAR, GREEN_CHAR, self.initial, RESET_CHAR)?;
        writeln!(f, "{}Finals   :{} {}{:?}{}\n\n", BOLD_PINK_CHAR, RESET_CHAR, GREEN_CHAR, self.finals, RESET_CHAR)?;
        write!(f, "\n{}\n\n", H_BORDER)?;

        for transitions in self.transitions.iter() {
            writeln!(f, "{}{:?}{} :\n", GREEN_CHAR, transitions.0, RESET_CHAR)?;
            for transition in transitions.1 {
                writeln!(f, "\t{}", transition)?;
            }
        }
        writeln!(f, "\n\n{}", H_BORDER)?;
        Ok(())
    }
}
