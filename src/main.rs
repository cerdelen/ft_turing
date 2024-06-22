use std::fs;
use clap::Parser;
use serde_json;

mod args;
mod types;
mod machine_input;
mod machine_description;

// use args::TuringMachineArgs;


fn main() {
    args::init();
}
