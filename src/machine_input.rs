use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub struct MachineInput {
}

impl MachineInput {
    pub fn new(input: BufReader<File>) -> Self {
        Self{}
    }
}
