mod vm;

use vm::{Vm, VmRegister};

fn main() -> Result<(), ()> {
    let lun = Vm::default();

    println!("Register a: {}", lun.get_register_value(VmRegister::a));

    Ok(())
}
