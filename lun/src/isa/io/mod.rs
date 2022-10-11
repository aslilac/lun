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
	cp_t_rr(VmRegister, VmRegister),
	mv_t_rr(VmRegister, VmRegister),
	pop_w_r(VmWordRegister),
	push_w_r(VmWordRegister),
	put_b_i(u8),
	put_b_r(VmByteRegister),
	put_w_r(VmWordRegister),
}

impl Instruction for IoInstruction {
	fn exec(self, vm: &mut Vm) {
		use IoInstruction::*;

		match self {
			cp_t_rr(r1, rr) => cp::t_rr(vm, r1, rr),
			mv_t_rr(r1, r2) => mv::t_rr(vm, r1, r2),
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
			cp_t_rr(r1, rr) => write!(disp, "mv {:?} {:?}", r1, rr),
			mv_t_rr(r1, rr) => write!(disp, "mv {:?} {:?}", r1, rr),
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

		vm.set_register_value(reg::x, 0x01);

		vm.exec(mv_t_rr(reg::xb0, reg::xb1));
		assert_eq!(vm.get_register_value(reg::x), 0x0100);

		vm.exec(mv_t_rr(reg::xq0, reg::xq1));
		assert_eq!(vm.get_register_value(reg::x), 0x01000000);
	}

	#[test]
	fn pop() {
		let mut vm = Vm::default();

		for i in 0..3 {
			vm.mem.push(i);
		}

		vm.exec(pop_w_r(reg::a.into()));
		vm.exec(pop_w_r(reg::b.into()));
		vm.exec(pop_w_r(reg::c.into()));

		assert_eq!(vm.get_register_value(reg::a), 0);
		assert_eq!(vm.get_register_value(reg::b), 1);
		assert_eq!(vm.get_register_value(reg::c), 2);
	}

	#[test]
	fn push() {
		let mut vm = Vm::default();

		vm.set_register_value(reg::s, 3);
		vm.set_register_value(reg::x, 1);
		vm.mem = vec![0; 3];

		vm.exec(push_w_r(reg::x.into()));
		assert_eq!(vm.mem[0], 0);
		assert_eq!(vm.mem[1], 0);
		assert_eq!(vm.mem[2], 1);

		vm.exec(push_w_r(reg::x.into()));
		assert_eq!(vm.mem[0], 0);
		assert_eq!(vm.mem[1], 1);
		assert_eq!(vm.mem[2], 1);

		vm.exec(push_w_r(reg::x.into()));
		assert_eq!(vm.mem[0], 1);
		assert_eq!(vm.mem[1], 1);
		assert_eq!(vm.mem[2], 1);

		assert_eq!(vm.get_register_value(reg::s), 0);
	}

	#[test]
	fn put() {
		let mut vm = Vm::default();

		vm.exec(put_b_i('a' as u8));
		assert_eq!(vm.disp.buffer()[0], 'a' as u8);
	}
}
