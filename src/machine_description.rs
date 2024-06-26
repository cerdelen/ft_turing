use serde::Deserializer;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

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
}
