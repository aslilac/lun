mod and;
mod cmp;
mod or;
mod xor;

use crate::vm::{Instruction, Vm, VmRegister};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LogicInstruction {
    and_rr(VmRegister, VmRegister),
    and_rrr(VmRegister, VmRegister, VmRegister),

    cmp_r(VmRegister),
    cmp_rr(VmRegister, VmRegister),

    or_rr(VmRegister, VmRegister),
    or_rrr(VmRegister, VmRegister, VmRegister),

    xor_r(VmRegister),
    xor_rr(VmRegister, VmRegister),
    xor_rrr(VmRegister, VmRegister, VmRegister),
}

impl Instruction for LogicInstruction {
    fn exec(self, vm: &mut Vm) {
        use LogicInstruction::*;

        match self {
            and_rr(r1, r2) => and::rrr(vm, r1, r2, r1),
            and_rrr(r1, r2, rr) => and::rrr(vm, r1, r2, rr),

            cmp_r(r1) => cmp::r(vm, r1),
            cmp_rr(r1, r2) => cmp::rr(vm, r1, r2),

            or_rr(r1, r2) => or::rrr(vm, r1, r2, r1),
            or_rrr(r1, r2, rr) => or::rrr(vm, r1, r2, rr),

            xor_r(r1) => xor::r(vm, r1),
            xor_rr(r1, r2) => xor::rrr(vm, r1, r2, r1),
            xor_rrr(r1, r2, r3) => xor::rrr(vm, r1, r2, r3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LogicInstruction::*, Vm, VmRegister::*};

    #[test]
    fn and() {
        let mut vm = Vm::default();

        vm.set_register_value(x, 7);
        vm.exec(and_rr(x, x));
        assert_eq!(vm.get_register_value(x), 7);

        vm.set_register_value(x, 0b10101);
        vm.set_register_value(y, 0b11111);
        vm.exec(and_rr(x, y));
        assert_eq!(vm.get_register_value(x), 0b10101);
    }

    #[test]
    fn xor() {
        let mut vm = Vm::default();

        vm.set_register_value(x, 7);
        vm.exec(xor_r(x));
        assert_eq!(vm.get_register_value(x), 0);

        vm.set_register_value(x, 0b10101);
        vm.set_register_value(y, 0b11111);
        vm.exec(xor_rr(x, y));
        assert_eq!(vm.get_register_value(x), 0b01010);
    }
}
