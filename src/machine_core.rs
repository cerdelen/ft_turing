use crate::consts::*;
use crate::machine_description::{DescriptionErrors, MachineDescription};
use crate::machine_tape::MachineTape;
use std::io::{self, Write};

pub struct MachineCore {
    description: MachineDescription,
    tape: MachineTape,
    state: usize,
    fast: bool,
}

impl MachineCore {
    pub fn new(
        desc: MachineDescription,
        tape: MachineTape,
        initial_state: String,
        fast: bool,
    ) -> Self {
        Self {
            description: desc,
            tape,
            state: initial_state,
            fast,
        }
    }
    #[allow(unused_assignments)]
    pub fn run(&mut self) {
        if self.fast {
            self.run_fast();
        } else {
            self.run_slow();
        }
    }

    fn run_slow(&mut self) {
        let mut buffer = String::new();
        let mut curr_head_pos: usize = 0;

        loop {
            curr_head_pos = self.tape.get_head_pos();
            buffer.push_str(&format!("{}⌲{}  [ ", BOLD_PINK_CHAR, RESET_CHAR));
            for (index, c) in self.tape.get_full_tape().iter().enumerate() {
                if curr_head_pos == index {
                    buffer.push_str(&format!("{}{}{}", ON_PINK_CHAR, *c, RESET_CHAR));
                } else {
                    buffer.push(*c);
                }
            }
            buffer.push_str(&format!(" ]\t "));
            let read = self.tape.get_read();
            buffer.push_str(&format!("( {}state:{} {:>20},  {}read:{} {} )",BOLD_YELLOW_CHAR, RESET_CHAR,
            self.description.get_state_name(self.state), BOLD_YELLOW_CHAR, RESET_CHAR, read));
            let trans = match self.description.get_transition(self.state, read) {
            buffer.push_str(&format!(
                "( {}state:{} {:>20},  {}read:{} {} )",
                BOLD_YELLOW_CHAR, RESET_CHAR, self.state, BOLD_YELLOW_CHAR, RESET_CHAR, read
            ));
            let trans = match self.description.get_transition(&self.state, read) {
                Ok(trans) => trans,
                Err(err) => {
                    match err {
                        DescriptionErrors::NoTransitionsForState =>
                            panic!("No TransitionsVector for Current State \"{}\"", self.description.get_state_name(self.state)),
                        DescriptionErrors::NoTransitionForReadInState =>
                            panic!("{}\n\n{}No Transition for Current Read in Transitionsvector for Current State: {}⌲ \"{}\"{}\n\n{}",
                            H_BORDER, BOLD_RED_CHAR, BOLD_YELLOW_CHAR, self.description.get_state_name(self.state), RESET_CHAR, H_BORDER),
                    }
                },
            };
            buffer.push_str(&format!(
                " \t->\t( {}write:{} {},  {}switch:{} {:>20}, {}action:{} {:?} )\n",
                BOLD_GREEN_CHAR, RESET_CHAR, trans.write, BOLD_GREEN_CHAR, RESET_CHAR,
                self.description.get_state_name(self.state), BOLD_GREEN_CHAR,  RESET_CHAR , trans.action
                BOLD_GREEN_CHAR,
                RESET_CHAR,
                trans.write,
                BOLD_GREEN_CHAR,
                RESET_CHAR,
                trans.to_state,
                BOLD_GREEN_CHAR,
                RESET_CHAR,
                trans.action
            ));
            self.tape.perform_write(&trans.write);
            self.tape.move_head(&trans.action);
            self.state = trans.to_state.clone();

            let stdout = io::stdout();
            let mut handle = stdout.lock();
            let _ = handle.write(&buffer.as_bytes());
            if self.description.check_for_end(self.state) == true {
                break;
            }
            buffer.clear();
        }
        println!(
            "\n\n{}⌲ Final Tape:{} {}\n\n",
            BOLD_GREEN_CHAR, RESET_CHAR, &self.tape
        );
    }

    fn run_fast(&mut self) {
        loop {
            let read = self.tape.get_read();
            let trans = match self.description.get_transition(&self.state, read) {
                Ok(trans) => trans,
                Err(err) => {
                    match err {
                        DescriptionErrors::NoTransitionsForState =>
                            panic!("No TransitionsVector for Current State \"{}\"", self.state),
                        DescriptionErrors::NoTransitionForReadInState =>
                            panic!("{}\n\n{}No Transition for Current Read in Transitionsvector for Current State: {}⌲ \"{}\"{}\n\n{}",
                            H_BORDER, BOLD_RED_CHAR, BOLD_YELLOW_CHAR, self.state, RESET_CHAR, H_BORDER),
                    }
                },
            };
            self.tape.perform_write(&trans.write);
            self.tape.move_head(&trans.action);
            self.state = trans.to_state.clone();

            if self.description.check_for_end(&self.state) == true {
                break;
            }
        }
        println!(
            "\n\n{}⌲ Final Tape:{} {}\n\n",
            BOLD_GREEN_CHAR, RESET_CHAR, &self.tape
        );
    }
}
