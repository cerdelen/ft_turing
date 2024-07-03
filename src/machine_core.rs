use std::io::{self, Write};

use crate::machine_description::{DescriptionErrors, MachineDescription};
use crate::machine_tape::MachineTape;

pub struct MachineCore {
    description: MachineDescription,
    tape: MachineTape,
    state: String,
}

impl MachineCore {
    pub fn new(desc: MachineDescription, tape: MachineTape, initial_state: String) -> Self {
        Self {
            description: desc,
            tape,
            state: initial_state,
        }
    }
    #[allow(unused_assignments)]
    pub fn run(&mut self) {
        let mut buffer = String::new();
        let mut curr_head_pos: usize = 0;

        loop {
            curr_head_pos = self.tape.get_head_pos();
            buffer.push_str(&format!("["));
            for (index, c) in self.tape.get_full_tape().iter().enumerate() {
                if curr_head_pos == index {
                    buffer.push_str(&format!("<{}>", *c));
                } else {
                    buffer.push(*c);
                }
            }
            buffer.push_str(&format!("] "));
            let read = self.tape.get_read();
            buffer.push_str(&format!("(state: {}, read: {}) ", self.state, read));
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
            buffer.push_str(&format!(
                "-> (write: {}, switch: {}, action: {:?})\n",
                trans.write, trans.to_state, trans.action
            ));
            self.tape.perform_write(&trans.write);
            self.tape.move_head(&trans.action);
            self.state = trans.to_state.clone();

            let stdout = io::stdout();
            let mut handle = stdout.lock();
            let _ = handle.write(&buffer.as_bytes());
            if self.description.check_for_end(&self.state) == true {
                return;
            }
            buffer.clear();
        }
    }
}
