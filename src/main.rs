use machine_core::MachineCore;

mod args;
mod machine_tape;
mod machine_description;
mod machine_core;
mod consts;
mod init_utm;

fn main() {
    let (desc, tape, initial_state) = args::init();
    let mut machine = MachineCore::new(desc, tape, initial_state);
    machine.run()
}
