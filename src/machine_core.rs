use crate::machine_description::MachineDescription;
use crate::machine_tape::MachineTape;

pub struct MachineCore {
    description: MachineDescription,
    tape: MachineTape,
    state: String,
}


impl MachineCore {

    pub fn new(desc: MachineDescription, tape: MachineTape, initial_state: String) -> Self {
        Self{description: desc, tape, state: initial_state}
    }

    pub fn run(&self) {
        let read = self.tape.get_read();
        let trans = self.description.get_transition(&self.state, read);

    }
}
