#![feature(mixed_integer_ops)]
// Remove ASAP
#![allow(dead_code)]

mod instruction_set;
mod prelude;
mod vm;

fn main() -> Result<(), std::io::Error> {
    use prelude::*;

    let mut vm = Vm::default();

    vm.set_register_value(a, 0x6173206f6c6c6548); // Hello sa
    vm.exec(put_r(a));
    vm.set_register_value(a, 0x0a21726f6c69); // ilor!\n
    vm.exec(put_r(a));

    // vm.inspect();

    Ok(())
}
