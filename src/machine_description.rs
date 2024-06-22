use std::error::Error;

pub struct MachineDescription {
}

impl MachineDescription {
    pub fn new(input: &str) -> Result<Self, Box<dyn Error>> {
        serde_json::from_str(input)?;
        Ok(MachineDescription{})
    }
}
