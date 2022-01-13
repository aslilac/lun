mod add;
mod mul;

use crate::vm::{Instruction, Vm, VmRegister};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArithmeticInstruction {
    add_i64_ri(VmRegister, i64),
    add_i64_rr(VmRegister, VmRegister),
    add_i64_rir(VmRegister, i64, VmRegister),
    add_i64_rrr(VmRegister, VmRegister, VmRegister),
    add_u64_ri(VmRegister, u64),
    add_u64_rr(VmRegister, VmRegister),
    add_u64_rir(VmRegister, u64, VmRegister),
    add_u64_rrr(VmRegister, VmRegister, VmRegister),

    mul_i64_ri(VmRegister, i64),
    mul_i64_rr(VmRegister, VmRegister),
    mul_i64_rir(VmRegister, i64, VmRegister),
    mul_i64_rrr(VmRegister, VmRegister, VmRegister),
    mul_u64_ri(VmRegister, u64),
    mul_u64_rr(VmRegister, VmRegister),
    mul_u64_rir(VmRegister, u64, VmRegister),
    mul_u64_rrr(VmRegister, VmRegister, VmRegister),
}

impl Instruction for ArithmeticInstruction {
    fn exec(self, vm: &mut Vm) {
        use ArithmeticInstruction::*;

        match self {
            add_i64_ri(r1, i) => add::i64_rir(vm, r1, i, r1),
            add_i64_rr(r1, r2) => add::i64_rrr(vm, r1, r2, r1),
            add_i64_rir(r1, i, rr) => add::i64_rir(vm, r1, i, rr),
            add_i64_rrr(r1, r2, rr) => add::i64_rrr(vm, r1, r2, rr),
            add_u64_ri(r1, i) => add::u64_rir(vm, r1, i, r1),
            add_u64_rr(r1, r2) => add::u64_rrr(vm, r1, r2, r1),
            add_u64_rir(r1, i, rr) => add::u64_rir(vm, r1, i, rr),
            add_u64_rrr(r1, r2, rr) => add::u64_rrr(vm, r1, r2, rr),

            mul_i64_ri(r1, i) => mul::i64_rir(vm, r1, i, r1),
            mul_i64_rr(r1, r2) => mul::i64_rrr(vm, r1, r2, r1),
            mul_i64_rir(r1, i, rr) => mul::i64_rir(vm, r1, i, rr),
            mul_i64_rrr(r1, r2, rr) => mul::i64_rrr(vm, r1, r2, rr),
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
    fn add_i64() {
        let mut vm = Vm::default();

        vm.exec(add_i64_ri(x, 2));
        assert_eq!(vm.get_register_value(x), 2);
        vm.exec(add_i64_rr(x, x));
        assert_eq!(vm.get_register_value(x), 4);

        vm.exec(add_i64_ri(x, -8));
        assert_eq!(vm.get_register_value(x) as i64, -4);
    }
    
    #[test]
    fn add_u64() {
        let mut vm = Vm::default();

        vm.exec(add_u64_ri(x, 2));
        assert_eq!(vm.get_register_value(x), 2);
        vm.exec(add_u64_rr(x, x));
        assert_eq!(vm.get_register_value(x), 4);
    }

    #[test]
    fn mul_i64() {
        let mut vm = Vm::default();

        vm.set_register_value(x, 5);

        vm.exec(mul_i64_ri(x, -1));
        assert_eq!(vm.get_register_value(x) as i64, -5);
        
        vm.exec(mul_i64_ri(x, 2));
        assert_eq!(vm.get_register_value(x) as i64, -10);

        vm.set_register_value(x, u64::MAX); // should be the same as -1
        vm.set_register_value(y, 5);

        vm.exec(mul_i64_rr(x, y));
        assert_eq!(vm.get_register_value(x) as i64, -5);

        vm.exec(mul_i64_rr(x, x));
        assert_eq!(vm.get_register_value(x) as i64, 25);
    }

    #[test]
    fn mul_u64() {
        let mut vm = Vm::default();

        vm.set_register_value(x, 2);

        vm.exec(mul_u64_rr(x, x));
        assert_eq!(vm.get_register_value(x), 4);
    }
}
