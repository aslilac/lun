use super::super::iprelude::*;

pub fn t_r(vm: &mut Vm, r1: VmRegister) {
	vm.set_register_value(r1, 0);
}

pub fn t_rrr(vm: &mut Vm, r1: VmRegister, r2: VmRegister, rr: VmRegister) {
	let r1v = vm.get_register_value(r1);
	let r2v = vm.get_register_value(r2);
	vm.set_register_value(rr, r1v ^ r2v);
}
