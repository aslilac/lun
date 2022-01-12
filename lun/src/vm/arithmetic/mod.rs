mod add;
mod mul;

use crate::vm::{Instruction, Vm, VmRegister};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArithmeticInstruction {
    add_u64_ri(VmRegister, u64),
    add_u64_rr(VmRegister, VmRegister),
    add_u64_rir(VmRegister, u64, VmRegister),
    add_u64_rrr(VmRegister, VmRegister, VmRegister),

    mul_u64_ri(VmRegister, u64),
    mul_u64_rr(VmRegister, VmRegister),
    mul_u64_rir(VmRegister, u64, VmRegister),
    mul_u64_rrr(VmRegister, VmRegister, VmRegister),
}

impl Instruction for ArithmeticInstruction {
    fn exec(self, vm: &mut Vm) {
        use ArithmeticInstruction::*;

        match self {
            add_u64_ri(r1, i) => add::u64_rir(vm, r1, i, r1),
            add_u64_rr(r1, r2) => add::u64_rrr(vm, r1, r2, r1),
            add_u64_rir(r1, i, rr) => add::u64_rir(vm, r1, i, rr),
            add_u64_rrr(r1, r2, rr) => add::u64_rrr(vm, r1, r2, rr),

            mul_u64_ri(r1, i) => mul::u64_rir(vm, r1, i, r1),
            mul_u64_rr(r1, r2) => mul::u64_rrr(vm, r1, r2, r1),
            mul_u64_rir(r1, i, rr) => mul::u64_rir(vm, r1, i, rr),
            mul_u64_rrr(r1, r2, rr) => mul::u64_rrr(vm, r1, r2, rr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ArithmeticInstruction::*, Vm, VmRegister::*};

    #[test]
    fn add() {
        let mut vm = Vm::default();

        vm.exec(add_u64_ri(x, 2));
        assert_eq!(vm.get_register_value(x), 2);
        vm.exec(add_u64_rr(x, x));
        assert_eq!(vm.get_register_value(x), 4);
    }

    fn mul() {
        let mut vm = Vm::default();
        
        vm.set_register_value(x, 2);

        vm.exec(mul_u64_rr(x, x));
        assert_eq!(vm.get_register_value(x), 4);
    }
}
