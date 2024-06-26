use crate::machine_description::{MachineDescription, DescriptionErrors};
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

    pub fn run(&mut self) {
        let read = self.tape.get_read();
        let trans = match self.description.get_transition(&self.state, read) {
            Ok(trans) => trans,
            Err(err) => {
                match err {
                    DescriptionErrors::NoTransitionsForState =>
                        panic!("No TransitionsVector for Current State \"{}\"", self.state),
                    DescriptionErrors::NoTransitionForReadInState =>
                        panic!("No Transition for Current Read in Transitionsvector for Current State \"{}\"", self.state),
                }
            },
        };
        self.tape.perform_write(&trans.write);
        match self.tape.move_head(&trans.action) {
            Ok(_) => (),
            Err(err) => {
                println!("Ran of the Tape!\n{:?}", err);
            },
        }
        self.state = trans.to_state.clone();
        // let transition = tansitions.iter().find(|&entry| entry.read == current_read);

    }
}
