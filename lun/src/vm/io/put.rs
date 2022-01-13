use crate::vm::Vm;
use std::io::Write;

pub fn i(_: &mut Vm, i: u8) {
    let _ = std::io::stdout().lock().write(&[i]);
}
