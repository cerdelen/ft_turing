use machine_core::MachineCore;

mod args;
mod machine_tape;
mod machine_description;
mod machine_core;

fn main() {
    let (desc, tape) = args::init();
    println!("Machine Tape: {:?}\nMachine Description: {:?}", tape, desc);
    let machine = MachineCore::new(desc, tape);
    machine.run()
}
