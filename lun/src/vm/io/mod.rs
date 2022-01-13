mod put;

use crate::vm::{Instruction, Vm, VmRegister};
use std::io;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IoInstruction {
    put_i(u8),
}

impl Instruction for IoInstruction {
    fn exec(self, vm: &mut Vm) {
        use IoInstruction::*;

        match self {
            put_i(i) => put::i(vm, i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{IoInstruction::*, Vm, VmRegister::*};

    #[test]
    fn put() {
        let mut vm = Vm::default();
    }
}
