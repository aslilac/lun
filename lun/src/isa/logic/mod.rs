mod and;
mod cmp;
mod or;
mod xor;

use super::iprelude::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LogicInstruction {
	and_t_rrr(VmRegister, VmRegister, VmRegister),
	// add other variants?
	cmp_w_r(VmWordRegister),
	cmp_w_rr(VmWordRegister, VmWordRegister),
	or_t_rrr(VmRegister, VmRegister, VmRegister),
	xor_t_r(VmRegister),
	xor_t_rrr(VmRegister, VmRegister, VmRegister),
}

impl Instruction for LogicInstruction {
	fn exec(self, vm: &mut Vm) {
		use LogicInstruction::*;

		match self {
			and_t_rrr(r1, r2, rr) => and::t_rrr(vm, r1, r2, rr),
			cmp_w_r(r1) => cmp::w_r(vm, r1),
			cmp_w_rr(r1, r2) => cmp::w_rr(vm, r1, r2),

			or_t_rrr(r1, r2, rr) => or::t_rrr(vm, r1, r2, rr),

			xor_t_r(r1) => xor::t_r(vm, r1),
			xor_t_rrr(r1, r2, r3) => xor::t_rrr(vm, r1, r2, r3),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{super::iprelude::*, LogicInstruction::*};

	#[test]
	fn and() {
		let mut vm = Vm::default();

		vm.set_register_value(reg::x, 7);
		vm.exec(and_t_rrr(reg::x, reg::x, reg::x));
		assert_eq!(vm.get_register_value(reg::x), 7);

		vm.set_register_value(reg::x, 0b10101);
		vm.set_register_value(reg::y, 0b11011);
		vm.exec(and_t_rrr(reg::x, reg::y, reg::x));
		assert_eq!(vm.get_register_value(reg::x), 0b10001);
	}

	#[test]
	fn xor() {
		let mut vm = Vm::default();

		vm.set_register_value(reg::x, 7);
		vm.exec(xor_t_r(reg::x));
		assert_eq!(vm.get_register_value(reg::x), 0);

		vm.set_register_value(reg::x, 0b10101);
		vm.set_register_value(reg::y, 0b11111);
		vm.exec(xor_t_rrr(reg::x, reg::y, reg::x));
		assert_eq!(vm.get_register_value(reg::x), 0b01010);
	}
}
