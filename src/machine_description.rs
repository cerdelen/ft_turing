use std::collections::HashMap;
use serde::Deserialize;
use std::io::BufReader;
use std::fs::File;

#[derive(Deserialize, Debug)]
pub struct Transitions {
    read: String,
    to_state: String,
    write: String,
    action: String,
}

#[derive(Deserialize, Debug)]
pub struct MachineDescription {
    name: String,
    alphabet: Vec<String>,
    blank: String,
    states: Vec<String>,
    initial: String,
    finals: Vec<String>,
    transitions: HashMap<String, Vec<Transitions>>,
}

impl MachineDescription {
    pub fn new(input: BufReader<File>) -> Self {
        serde_json::from_reader(input).expect("Invalid Json")
    }
}
