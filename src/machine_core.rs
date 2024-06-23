use crate::{machine_description::MachineDescription, machine_input::MachineTape};

pub struct MachineCore {
    description: MachineDescription,
    tape: MachineTape,
}


impl MachineCore {

    pub fn new(desc: MachineDescription, tape: MachineTape) -> Self {
        Self{description: desc, tape}

    }

    pub fn run(&self) {

    }
}
