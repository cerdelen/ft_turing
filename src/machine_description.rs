use std::error::Error;
use std::io::BufReader;
use std::fs::File;

use serde_json::Value;

pub struct MachineDescription {
}

impl MachineDescription {
    pub fn new(input: BufReader<File>) -> Self {
         let _: Value = serde_json::from_reader(input).expect("Invalid Json");
        MachineDescription{}
    }
}
