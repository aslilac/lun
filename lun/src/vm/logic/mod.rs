mod xor;

use crate::vm::{Instruction, Vm, VmRegister};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LogicInstruction {
    xor_u64_r(VmRegister),
    xor_u64_rr(VmRegister, VmRegister),
    xor_u64_rrr(VmRegister, VmRegister, VmRegister),
}

impl Instruction for LogicInstruction {
    fn exec(self, vm: &mut Vm) {
        use LogicInstruction::*;

        match self {
            xor_u64_r(r1) => xor::xor_r(vm, r1),
            xor_u64_rr(r1, r2) => xor::xor_rrr(vm, r1, r2, r1),
            xor_u64_rrr(r1, r2, r3) => xor::xor_rrr(vm, r1, r2, r3),
        }
    }
}
