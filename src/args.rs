use clap:: {
    Args,
    Parser,
    Subcommand
};

use std::error::Error;
use std::path::Path;
use std::fs;
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

    let raw_machine_input = fs::read_to_string(Path::new(&args.machine_input)).expect(&format!("Could not read from {:?}", &args.machine_input));
    let desc: MachineDescription = MachineDescription::new("lol")?;
    let raw_machine_description = fs::read_to_string(Path::new(&args.machine_description)).expect(&format!("Could not read from {:?}", &args.machine_description));
    let input: MachineInput = MachineInput::new("lol")?;
    println!("Machine Input: {}\nMachine Description: {}", args.machine_input, args.machine_description);
    Ok((desc, input))
}
