#![feature(mixed_integer_ops)]
#![feature(trait_alias)]
// Remove ASAP
#![allow(dead_code)]

mod base;
mod isa;
mod prelude;
mod vm;

fn main() -> Result<(), std::io::Error> {
    use prelude::*;

    let mut vm = Vm::default();

    vm.set_register_value(a, 0x6173206f_6c6c6548); // Hello sa
    vm.exec(put_w_r(a));
    vm.set_register_value(a, 0x0a21_726f6c69); // ilor!\n
    vm.exec(put_w_r(a));

    Ok(())
}
