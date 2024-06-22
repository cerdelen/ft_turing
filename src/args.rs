use clap:: {
    Args,
    Parser,
    Subcommand
};

use std::error::Error;
use std::path::Path;
use std::fs::{self, File};
use std::io::BufReader;
// use crate::types::{MachineInput, MachineDescription};
use crate::machine_description::MachineDescription;
use crate::machine_input::MachineInput;


#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct TuringMachineArgs {
    pub machine_description: String,
    pub machine_input: String,
}


pub fn init() -> Result<(MachineDescription, MachineInput), Box<dyn Error>> {
    let args = TuringMachineArgs::parse();

    let m_i_file = File::open(Path::new(&args.machine_input)).expect(&format!("Could not open Machine Input File \"{:?}\"", &args.machine_input));
    let machine_input_reader = BufReader::new(m_i_file);
    let input: MachineInput = MachineInput::new(machine_input_reader);

    let m_d_file = File::open(Path::new(&args.machine_description)).expect(&format!("Could not open Machine Description File \"{:?}\"", &args.machine_description));
    let machine_description_reader = BufReader::new(m_d_file);
    let desc: MachineDescription = MachineDescription::new(machine_description_reader);

    println!("Machine Input: {}\nMachine Description: {}", args.machine_input, args.machine_description);
    Ok((desc, input))
}
