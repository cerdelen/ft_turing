use std::collections::HashMap;
use serde::Deserialize;
use std::io::BufReader;
use std::fs::File;

pub enum DescriptionErrors {
    NoTransitionsForState,
    NoTransitionForReadInState,
}


#[derive(Deserialize, Debug)]
pub struct Transitions {
    read: char,
    to_state: String,
    write: char,
    action: String,
}

#[derive(Deserialize, Debug)]
pub struct MachineDescription {
    name: String,
    alphabet: Vec<char>,
    blank: char,
    states: Vec<String>,
    initial: String,
    finals: Vec<String>,
    transitions: HashMap<String, Vec<Transitions>>,
}

impl MachineDescription {
    pub fn new(input: BufReader<File>) -> Self {
        serde_json::from_reader(input).expect("Invalid Json")
    }

    // get all Transitions for given State
    fn get_transition_vec(&self, state: &String) -> Option<&Vec<Transitions>> {
        self.transitions.get(state)
    }

    // get Transition for given State AND current read
    pub fn get_transition(&self, state: &String, current_read: char) -> Result<&Transitions, DescriptionErrors> {
        let tansitions = self.get_transition_vec(state).ok_or(DescriptionErrors::NoTransitionsForState)?;
        let transition = tansitions.iter().find(|&entry| entry.read == current_read);
        let ret  = transition.ok_or(DescriptionErrors::NoTransitionForReadInState)?;
        Ok(ret)
    }
}
