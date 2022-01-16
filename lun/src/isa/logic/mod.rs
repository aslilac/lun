mod and;
mod cmp;
mod or;
mod xor;

use super::iprelude::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LogicInstruction {
    and_b_rr(VmByteRegister, VmByteRegister),
    and_b_rrr(VmByteRegister, VmByteRegister, VmByteRegister),
    and_q_rr(VmQwordRegister, VmQwordRegister),
    and_q_rrr(VmQwordRegister, VmQwordRegister, VmQwordRegister),
    and_h_rr(VmHwordRegister, VmHwordRegister),
    and_h_rrr(VmHwordRegister, VmHwordRegister, VmHwordRegister),
    and_w_rr(VmNativeRegister, VmNativeRegister),
    and_w_rrr(VmNativeRegister, VmNativeRegister, VmNativeRegister),

    // cmp_b_r(VmByteRegister),
    // cmp_b_rr(VmByteRegister, VmByteRegister),
    // cmp_q_r(VmQwordRegister),
    // cmp_q_rr(VmQwordRegister, VmQwordRegister),
    // cmp_h_r(VmHwordRegister),
    // cmp_h_rr(VmHwordRegister, VmHwordRegister),
    cmp_w_r(VmNativeRegister),
    cmp_w_rr(VmNativeRegister, VmNativeRegister),

    or_b_rr(VmByteRegister, VmByteRegister),
    or_b_rrr(VmByteRegister, VmByteRegister, VmByteRegister),
    or_q_rr(VmQwordRegister, VmQwordRegister),
    or_q_rrr(VmQwordRegister, VmQwordRegister, VmQwordRegister),
    or_h_rr(VmHwordRegister, VmHwordRegister),
    or_h_rrr(VmHwordRegister, VmHwordRegister, VmHwordRegister),
    or_w_rr(VmNativeRegister, VmNativeRegister),
    or_w_rrr(VmNativeRegister, VmNativeRegister, VmNativeRegister),

    xor_b_r(VmByteRegister),
    xor_b_rr(VmByteRegister, VmByteRegister),
    xor_b_rrr(VmByteRegister, VmByteRegister, VmByteRegister),
    xor_q_r(VmQwordRegister),
    xor_q_rr(VmQwordRegister, VmQwordRegister),
    xor_q_rrr(VmQwordRegister, VmQwordRegister, VmQwordRegister),
    xor_h_r(VmHwordRegister),
    xor_h_rr(VmHwordRegister, VmHwordRegister),
    xor_h_rrr(VmHwordRegister, VmHwordRegister, VmHwordRegister),
    xor_w_r(VmNativeRegister),
    xor_w_rr(VmNativeRegister, VmNativeRegister),
    xor_w_rrr(VmNativeRegister, VmNativeRegister, VmNativeRegister),
}

impl Instruction for LogicInstruction {
    fn exec(self, vm: &mut Vm) {
        use LogicInstruction::*;

        match self {
            and_b_rr(r1, r2) => and::t_rrr(vm, r1, r2, r1),
            and_b_rrr(r1, r2, rr) => and::t_rrr(vm, r1, r2, rr),
            and_q_rr(r1, r2) => and::t_rrr(vm, r1, r2, r1),
            and_q_rrr(r1, r2, rr) => and::t_rrr(vm, r1, r2, rr),
            and_h_rr(r1, r2) => and::t_rrr(vm, r1, r2, r1),
            and_h_rrr(r1, r2, rr) => and::t_rrr(vm, r1, r2, rr),
            and_w_rr(r1, r2) => and::t_rrr(vm, r1, r2, r1),
            and_w_rrr(r1, r2, rr) => and::t_rrr(vm, r1, r2, rr),

            // cmp_b_r(r1) => cmp::t_r(vm, r1),
            // cmp_b_rr(r1, r2) => cmp::t_rr(vm, r1, r2),
            // cmp_q_r(r1) => cmp::t_r(vm, r1),
            // cmp_q_rr(r1, r2) => cmp::t_rr(vm, r1, r2),
            // cmp_h_r(r1) => cmp::t_r(vm, r1),
            // cmp_h_rr(r1, r2) => cmp::t_rr(vm, r1, r2),
            cmp_w_r(r1) => cmp::w_r(vm, r1),
            cmp_w_rr(r1, r2) => cmp::w_rr(vm, r1, r2),

            or_b_rr(r1, r2) => or::t_rrr(vm, r1, r2, r1),
            or_b_rrr(r1, r2, rr) => or::t_rrr(vm, r1, r2, rr),
            or_q_rr(r1, r2) => or::t_rrr(vm, r1, r2, r1),
            or_q_rrr(r1, r2, rr) => or::t_rrr(vm, r1, r2, rr),
            or_h_rr(r1, r2) => or::t_rrr(vm, r1, r2, r1),
            or_h_rrr(r1, r2, rr) => or::t_rrr(vm, r1, r2, rr),
            or_w_rr(r1, r2) => or::t_rrr(vm, r1, r2, r1),
            or_w_rrr(r1, r2, rr) => or::t_rrr(vm, r1, r2, rr),

            xor_b_r(r1) => xor::t_r(vm, r1),
            xor_b_rr(r1, r2) => xor::t_rrr(vm, r1, r2, r1),
            xor_b_rrr(r1, r2, rr) => xor::t_rrr(vm, r1, r2, rr),
            xor_q_r(r1) => xor::t_r(vm, r1),
            xor_q_rr(r1, r2) => xor::t_rrr(vm, r1, r2, r1),
            xor_q_rrr(r1, r2, rr) => xor::t_rrr(vm, r1, r2, rr),
            xor_h_r(r1) => xor::t_r(vm, r1),
            xor_h_rr(r1, r2) => xor::t_rrr(vm, r1, r2, r1),
            xor_h_rrr(r1, r2, rr) => xor::t_rrr(vm, r1, r2, rr),
            xor_w_r(r1) => xor::t_r(vm, r1),
            xor_w_rr(r1, r2) => xor::t_rrr(vm, r1, r2, r1),
            xor_w_rrr(r1, r2, r3) => xor::t_rrr(vm, r1, r2, r3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::iprelude::*, LogicInstruction::*};

    #[test]
    fn and() {
        let mut vm = Vm::default();

        vm.set_register_value(x, 7);
        vm.exec(and_w_rr(x, x));
        assert_eq!(vm.get_register_value(x), 7);

        vm.set_register_value(x, 0b10101);
        vm.set_register_value(y, 0b11111);
        vm.exec(and_w_rr(x, y));
        assert_eq!(vm.get_register_value(x), 0b10101);
    }

    #[test]
    fn xor() {
        let mut vm = Vm::default();

        vm.set_register_value(x, 7);
        vm.exec(xor_w_r(x));
        assert_eq!(vm.get_register_value(x), 0);

        vm.set_register_value(x, 0b10101);
        vm.set_register_value(y, 0b11111);
        vm.exec(xor_w_rr(x, y));
        assert_eq!(vm.get_register_value(x), 0b01010);
    }
}
