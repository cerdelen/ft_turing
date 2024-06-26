use clap:: Parser;

use std::path::Path;
use std::fs::File;
use std::io::BufReader;

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

    println!("{}", desc);
    // desc.print_full_description();
    // println!("Machine Description: {:?}", tape, desc);

    let m_i_file = File::open(Path::new(&args.machine_tape)).expect(&format!("Could not open Machine Input File \"{:?}\"", &args.machine_tape));
    let mut machine_input_reader = BufReader::new(m_i_file);
    let input: MachineTape = MachineTape::new(&mut machine_input_reader, &desc);

    println!("Machine Tape: {:?}", &input);
    println!("********************************************************************************");

    (desc, input, initial_state)
}
