mod pop;
mod push;
mod put;

use super::iprelude::*;
use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IoInstruction {
    pop_r(VmRegister),
    push_r(VmRegister),
    put_i(u8),
    put_r(VmRegister),
}

impl Instruction for IoInstruction {
    fn exec(self, vm: &mut Vm) {
        use IoInstruction::*;

        match self {
            pop_r(r1) => pop::r(vm, r1),
            push_r(r1) => push::r(vm, r1),
            put_i(i) => put::i(vm, i),
            put_r(r1) => put::r(vm, r1),
        }
    }
}

impl fmt::Display for IoInstruction {
    fn fmt(&self, disp: &mut fmt::Formatter<'_>) -> fmt::Result {
        use IoInstruction::*;

        match self {
            pop_r(r1) => write!(disp, "pop {:?}", r1),
            push_r(r1) => write!(disp, "push {:?}", r1),
            put_i(i) => write!(disp, "put {}", *i as char),
            put_r(r1) => write!(disp, "put {:?}", r1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::iprelude::*, IoInstruction::*};

    #[test]
    fn pop() {
        let mut vm = Vm::default();

        for i in 0..3 {
            vm.mem.push(i);
        }

        vm.exec(pop_r(a));
        vm.exec(pop_r(b));
        vm.exec(pop_r(c));

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

        vm.exec(push_r(x));
        assert_eq!(vm.mem[0], 0);
        assert_eq!(vm.mem[1], 0);
        assert_eq!(vm.mem[2], 1);

        vm.exec(push_r(x));
        assert_eq!(vm.mem[0], 0);
        assert_eq!(vm.mem[1], 1);
        assert_eq!(vm.mem[2], 1);

        vm.exec(push_r(x));
        assert_eq!(vm.mem[0], 1);
        assert_eq!(vm.mem[1], 1);
        assert_eq!(vm.mem[2], 1);

        assert_eq!(vm.get_register_value(s), 0);
    }

    #[test]
    fn put() {
        let mut vm = Vm::default();

        vm.exec(put_i('a' as u8));
        assert_eq!(vm.dsp_buf.0[0], 'a' as u8);
        vm.dsp_buf.clear();
    }
}
