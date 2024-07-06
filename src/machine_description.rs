use serde::Deserializer;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
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
    pub read: char,
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
struct InitTransition {
    pub read: char,
    pub to_state: String,
    pub write: char,
    pub action: Action,
}

#[derive(Deserialize, Debug)]
struct InitMachineDescription {
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
        let ret: InitMachineDescription = serde_json::from_reader(input)
                                        .expect(&format!("\n{}\n\n\t\t{}Invalid Machine Description File!{}\n\n{}\n\n",
                                        H_BORDER, BOLD_RED_CHAR, RESET_CHAR, H_BORDER));
        let ret: Self = MachineDescription::transform(&ret);
        let initial = ret.initial;
        (ret, initial)
    }


    fn transform(init_desc: &InitMachineDescription) -> Self {
        let mut unique_check: HashSet<String> = HashSet::new();
        for state in &init_desc.states {
            match unique_check.insert(state.clone()) {
                true => (),
                false => panic!("{}", &format_error_message("\tDuplicate State in \"states\" Array", state)),
            }
        }
        let mut finals = Vec::new();
        for finalstate in &init_desc.finals {
            finals.push(init_desc.states.iter().position(|s| s == finalstate)
                .expect(&format_error_message("Statename in Finals Array not found ", &finalstate)));
        };
        let mut transitions: Vec<HashMap<char, Transition>> = Vec::new();
        transitions.reserve(init_desc.states.len());
        for _ in 1..init_desc.states.len() {
            transitions.push(HashMap::new());
        }
        let initial = init_desc.states.iter().position(|s| s == &init_desc.initial)
            .expect(&format_error_message("Statename for initial State not found", &init_desc.initial));

        for transition_state in &init_desc.transitions {
            let statename_idx = init_desc.states.iter().position(|s| s == transition_state.0)
                .expect(&format_error_message("Statename for Transition an Array not found", &transition_state.0));
            let mut transitions_map: HashMap<char, Transition> = HashMap::new();
            for transition in transition_state.1 {
                let to_statename_idx = init_desc.states.iter().position(|s| s == &transition.to_state)
                    .expect(&format_error_message("Statename in Transition Declaration not found", &transition.to_state));
                transitions_map.insert(transition.read, Transition { read: transition.read, to_state: to_statename_idx, write: transition.write, action: transition.action.clone() });
            }
            transitions[statename_idx] = transitions_map;
        }
        Self {
            name: init_desc.name.to_owned(),
            alphabet: init_desc.alphabet.to_owned(),
            blank: init_desc.blank.to_owned(),
            states: init_desc.states.to_owned(),
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
        self.states.get(state)
        .expect(&format!("\n{}\n\n\t\t{}Trying to get Statename outsife of state Vector{}\n\n{}\n\n",
        H_BORDER, BOLD_RED_CHAR, RESET_CHAR, H_BORDER))
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
        let mut finals:Vec<String> = Vec::new();
        self.finals.iter().for_each(|final_state| {
            finals.push(self.get_state_name(*final_state).clone());
        });

        writeln!(f, "{}Finals   :{} {}{:?}{}\n", BOLD_PINK_CHAR, RESET_CHAR, GREEN_CHAR, finals, RESET_CHAR)?;
        writeln!(f, "{}Blank    :{} {}{:?}{}\n\n", BOLD_PINK_CHAR, RESET_CHAR, GREEN_CHAR, self.blank, RESET_CHAR)?;
        write!(f, "\n{}\n\n", H_BORDER)?;

        for (i, transitions) in self.transitions.iter().enumerate() {
            writeln!(f, "{}{:?}{} :\n", GREEN_CHAR, self.get_state_name(i), RESET_CHAR)?;
            for transition in transitions {
                write!(f, "\t\'{}{}{}\' -> (write: \'{}{}{}\', action: {}{:?}{}, change_to: {}{}{})\n",
                BOLD_PINK_CHAR, transition.1.read, RESET_CHAR, CYAN_CHAR, transition.1.write, RESET_CHAR, CYAN_CHAR,
                transition.1.action, RESET_CHAR, CYAN_CHAR, self.get_state_name(transition.1.to_state), RESET_CHAR)?;
            }
        write!(f, "\n")?;
    }
        writeln!(f, "\n\n{}", H_BORDER)?;
        Ok(())
    }
}
