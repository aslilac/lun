mod cp;
mod mv;
mod pop;
mod push;
mod put;

use super::iprelude::*;
use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IoInstruction {
    cp_b_rr(VmByteRegister, VmByteRegister),
    cp_q_rr(VmQwordRegister, VmQwordRegister),
    cp_h_rr(VmHwordRegister, VmHwordRegister),
    cp_w_rr(VmNativeRegister, VmNativeRegister),
    mv_b_rr(VmByteRegister, VmByteRegister),
    mv_q_rr(VmQwordRegister, VmQwordRegister),
    mv_h_rr(VmHwordRegister, VmHwordRegister),
    mv_w_rr(VmNativeRegister, VmNativeRegister),
    pop_w_r(VmNativeRegister),
    push_w_r(VmNativeRegister),
    put_b_i(u8),
    put_b_r(VmByteRegister),
    put_w_r(VmNativeRegister),
}

impl Instruction for IoInstruction {
    fn exec(self, vm: &mut Vm) {
        use IoInstruction::*;

        match self {
            cp_b_rr(r1, rr) => cp::t_rr(vm, r1, rr),
            cp_q_rr(r1, rr) => cp::t_rr(vm, r1, rr),
            cp_h_rr(r1, rr) => cp::t_rr(vm, r1, rr),
            cp_w_rr(r1, rr) => cp::t_rr(vm, r1, rr),
            mv_b_rr(r1, rr) => mv::t_rr(vm, r1, rr),
            mv_q_rr(r1, rr) => mv::t_rr(vm, r1, rr),
            mv_h_rr(r1, rr) => mv::t_rr(vm, r1, rr),
            mv_w_rr(r1, rr) => mv::t_rr(vm, r1, rr),
            pop_w_r(r1) => pop::w_r(vm, r1),
            push_w_r(r1) => push::w_r(vm, r1),
            put_b_i(i) => put::b_i(vm, i),
            put_b_r(r1) => put::b_r(vm, r1),
            put_w_r(r1) => put::w_r(vm, r1),
        }
    }
}

impl fmt::Display for IoInstruction {
    fn fmt(&self, disp: &mut fmt::Formatter<'_>) -> fmt::Result {
        use IoInstruction::*;

        match self {
            cp_b_rr(r1, rr) => write!(disp, "mv {:?} {:?}", r1, rr),
            cp_q_rr(r1, rr) => write!(disp, "mv {:?} {:?}", r1, rr),
            cp_h_rr(r1, rr) => write!(disp, "mv {:?} {:?}", r1, rr),
            cp_w_rr(r1, rr) => write!(disp, "mv {:?} {:?}", r1, rr),
            mv_b_rr(r1, rr) => write!(disp, "mv {:?} {:?}", r1, rr),
            mv_q_rr(r1, rr) => write!(disp, "mv {:?} {:?}", r1, rr),
            mv_h_rr(r1, rr) => write!(disp, "mv {:?} {:?}", r1, rr),
            mv_w_rr(r1, rr) => write!(disp, "mv {:?} {:?}", r1, rr),
            pop_w_r(r1) => write!(disp, "pop {:?}", r1),
            push_w_r(r1) => write!(disp, "push {:?}", r1),
            put_b_i(i) => write!(disp, "put {}", *i as char),
            put_b_r(r1) => write!(disp, "put {:?}", r1),
            put_w_r(r1) => write!(disp, "put {:?}", r1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::iprelude::*, IoInstruction::*};

    #[test]
    fn mv() {
        let mut vm = Vm::default();

        let xb0 = VmByteRegister::new(x, 0);
        let xb1 = VmByteRegister::new(x, 1);
        vm.set_register_value(x, 0x01);

        vm.exec(mv_b_rr(xb0, xb1));
        assert_eq!(vm.get_register_value(x), 0x0101);

        let xq0 = VmQwordRegister::new(x, 0);
        let xq1 = VmQwordRegister::new(x, 1);

        vm.exec(mv_q_rr(xq0, xq1));
        assert_eq!(vm.get_register_value(x), 0x01010101);
    }

    #[test]
    fn pop() {
        let mut vm = Vm::default();

        for i in 0..3 {
            vm.mem.push(i);
        }

        vm.exec(pop_w_r(a));
        vm.exec(pop_w_r(b));
        vm.exec(pop_w_r(c));

        assert_eq!(vm.get_register_value(a), 0);
        assert_eq!(vm.get_register_value(b), 1);
        assert_eq!(vm.get_register_value(c), 2);
    }

    #[test]
    fn push() {
        let mut vm = Vm::default();

        vm.set_register_value(s, 3);
        vm.set_register_value(x, 1);
        vm.mem = vec![0; 3];

        vm.exec(push_w_r(x));
        assert_eq!(vm.mem[0], 0);
        assert_eq!(vm.mem[1], 0);
        assert_eq!(vm.mem[2], 1);

        vm.exec(push_w_r(x));
        assert_eq!(vm.mem[0], 0);
        assert_eq!(vm.mem[1], 1);
        assert_eq!(vm.mem[2], 1);

        vm.exec(push_w_r(x));
        assert_eq!(vm.mem[0], 1);
        assert_eq!(vm.mem[1], 1);
        assert_eq!(vm.mem[2], 1);

        assert_eq!(vm.get_register_value(s), 0);
    }

    #[test]
    fn put() {
        let mut vm = Vm::default();

        vm.exec(put_b_i('a' as u8));
        assert_eq!(vm.disp.buffer()[0], 'a' as u8);
    }
}
