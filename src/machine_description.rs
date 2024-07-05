use serde::Deserializer;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::usize;
use crate::consts::*;

pub enum DescriptionErrors {
    NoTransitionsForState,
    NoTransitionForReadInState,
}

#[derive(Debug, Clone)]
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
    pub to_state: usize,
    pub write: char,
    pub action: Action,
}

#[derive(Deserialize, Debug)]
pub struct MachineDescription {
    name: String,
    alphabet: Vec<char>,
    blank: char,
    states: Vec<String>,
    initial: usize,
    finals: Vec<usize>,
    transitions: Vec<HashMap<char, Transition>>,
}

#[derive(Deserialize, Debug)]
pub struct InitTransition {
    pub read: char,
    pub to_state: String,
    pub write: char,
    pub action: Action,
}

#[derive(Deserialize, Debug)]
pub struct InitMachineDescription {
    pub name: String,
    pub alphabet: Vec<char>,
    pub blank: char,
    pub states: Vec<String>,
    pub initial: String,
    pub finals: Vec<String>,
    pub transitions: HashMap<String, Vec<InitTransition>>,
}

impl MachineDescription {
    pub fn new(input: BufReader<File>) -> (Self, usize) {
        let ret: InitMachineDescription = serde_json::from_reader(input).expect("Invalid Json");
        let ret: Self = MachineDescription::transform(&ret);
        let initial = ret.initial;
        (ret, initial)
    }


    fn transform(initDesc: &InitMachineDescription) -> Self {
        let mut finals = Vec::new();
        for finalstate in &initDesc.finals {
            finals.push(initDesc.states.iter().position(|s| s == finalstate).expect(&format!("Statename not found {}", finalstate)));
        };
        let mut transitions: Vec<HashMap<char, Transition>> = Vec::new();
        transitions.reserve(initDesc.states.len());
        for n in 1..initDesc.states.len() {
            transitions.push(HashMap::new());
        }
        let initial = initDesc.states.iter().position(|s| s == &initDesc.initial).expect(&format!("Statename not found {}", &initDesc.initial));
        for transition_state in &initDesc.transitions {
            let statename_idx = initDesc.states.iter().position(|s| s == transition_state.0).expect(&format!("Statename not found {}", &transition_state.0));
            let mut transitions_map: HashMap<char, Transition> = HashMap::new();
            for transition in transition_state.1 {
                let to_statename_idx = initDesc.states.iter().position(|s| s == &transition.to_state).expect(&format!("Statename not found {}", &transition_state.0));
                transitions_map.insert(transition.read, Transition { read: transition.read, to_state: to_statename_idx, write: transition.write, action: transition.action.clone() });
            }
            transitions[statename_idx] = transitions_map;
        }
        Self {
            name: initDesc.name.to_owned(),
            alphabet: initDesc.alphabet.to_owned(),
            blank: initDesc.blank.to_owned(),
            states: initDesc.states.to_owned(),
            finals,
            transitions,
            initial,
        }
    }

    // get all Transitions for given State
    fn get_transition_vec(&self, state: usize) -> Option<&HashMap<char, Transition>> {
        self.transitions.get(state)
    }

    // get Transition for given State AND current read
    pub fn get_transition(
        &self,
        state: usize,
        current_read: &char,
    ) -> Result<&Transition, DescriptionErrors> {
        let tansitions = self
            .get_transition_vec(state)
            .ok_or(DescriptionErrors::NoTransitionsForState)?;
        let transition = tansitions.get(current_read);
        let ret = transition.ok_or(DescriptionErrors::NoTransitionForReadInState)?;
        Ok(ret)
    }

    pub fn check_for_end(&self, state: usize) -> bool {
        self.finals.contains(&state)
    }

    pub fn part_of_alphabet(&self, c: &char) -> bool {
        self.alphabet.contains(c)
    }

    pub fn get_blank(&self) -> char {
        self.blank
    }

    pub fn get_state_name(&self, state: usize) -> &String {
        self.states.get(state).expect("Trying to get Statename outsife of state Vector")
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
        writeln!(f, "{}Initial  :{} {}{:?}{}\n", BOLD_PINK_CHAR, RESET_CHAR, GREEN_CHAR, self.get_state_name(self.initial), RESET_CHAR)?;
        writeln!(f, "{}Finals   :{} {}{:?}{}\n", BOLD_PINK_CHAR, RESET_CHAR, GREEN_CHAR, self.finals, RESET_CHAR)?;
        writeln!(f, "{}Blank    :{} {}{:?}{}\n\n", BOLD_PINK_CHAR, RESET_CHAR, GREEN_CHAR, self.blank, RESET_CHAR)?;
        write!(f, "\n{}\n\n", H_BORDER)?;

        for (i, transitions) in self.transitions.iter().enumerate() {
            writeln!(f, "{}{:?}{} :\n", GREEN_CHAR, self.get_state_name(i), RESET_CHAR)?;
            for transition in transitions {
                writeln!(f, "\t{}", transition.1)?;
            }
        }
        writeln!(f, "\n\n{}", H_BORDER)?;
        Ok(())
    }
}
