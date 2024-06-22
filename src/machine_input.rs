use std::error::Error;

pub struct MachineInput {
}

impl MachineInput {
    pub fn new(input: &str) -> Result<Self, Box<dyn Error>> {
        Ok(Self{})
    }
}
