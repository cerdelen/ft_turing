use std::fs::File;
use std::io::{BufReader, Read};

use crate::consts::UTM_ENCODING_TO_INPUT_DELIMITER;
use crate::machine_description::MachineDescription;
use crate::machine_tape::MachineTape;


impl MachineDescription {
    pub fn init_from_utm(desc_str: String) -> (Self, String) {
        todo!()
    }
}

impl MachineTape {
    pub fn new_utm(input: &mut BufReader<File>) -> (MachineDescription, Self, String) {
        let mut content = String::new();
        input.read_to_string(&mut content).expect("Could not Read Input Tape");
        // deleting new line
        if content.ends_with('\n') {
            content.pop();
        }

        let mut result: Vec<String> = content.split(UTM_ENCODING_TO_INPUT_DELIMITER)
                                            .map(|s| s.to_string())
                                            .collect();

        if result.len() != 2 {
            panic!("More than 1 UTM_ENCODING_TO_INPUT_DELIMITER!  Current Delimiter is: {}", UTM_ENCODING_TO_INPUT_DELIMITER);
        }
        let desc_str: String = result.remove(0);
        let tape_str: String = result.remove(0);

        let (desc, init_state): (MachineDescription, String) = MachineDescription::init_from_utm(desc_str);
        // divide with delimiter into M and w
        //
        // load machine
        //
        //
        //
        //
        //
        // initialize rest of tape_str

        let tape = MachineTape::init(tape_str, &desc);
        (desc, tape, init_state)





    }
}
