use clap:: Parser;

use std::path::Path;
use std::fs::File;
use std::io::BufReader;

use crate::consts::{CYAN_CHAR, H_BORDER, RESET_CHAR};
use crate::machine_description::MachineDescription;
use crate::machine_tape::MachineTape;


#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct TuringMachineArgs {
    pub machine_description: String,
    pub machine_tape: String,
}


pub fn init() -> (MachineDescription, MachineTape, String) {
    let args = TuringMachineArgs::parse();

    let m_d_file = File::open(Path::new(&args.machine_description)).expect(&format!("Could not open Machine Description File \"{:?}\"", &args.machine_description));
    let machine_description_reader = BufReader::new(m_d_file);
    let (desc, initial_state): (MachineDescription, String) = MachineDescription::new(machine_description_reader);

    // This is official description print, overloaded
    println!("{}", desc);

    let m_i_file = File::open(Path::new(&args.machine_tape)).expect(&format!("Could not open Machine Input File \"{:?}\"", &args.machine_tape));
    let mut machine_input_reader = BufReader::new(m_i_file);
    let input: MachineTape = MachineTape::new(&mut machine_input_reader, &desc);

    // This is official starting Tape print, overloaded
    println!("{}Input Tape:{} {}\n",CYAN_CHAR, RESET_CHAR, &input);
    println!("{}\n\n",H_BORDER);

    (desc, input, initial_state)
}
