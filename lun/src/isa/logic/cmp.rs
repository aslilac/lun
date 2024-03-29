use super::super::iprelude::*;

pub fn w_r(vm: &mut Vm, r1: VmWordRegister) {
	let r1v = vm.get_register_value(r1.into());
	vm.eq = r1v == 0;
	vm.ng = (r1v & (1 << 63)) > 0;
	vm.ov = false;
}

pub fn w_rr(vm: &mut Vm, r1: VmWordRegister, r2: VmWordRegister) {
	let r1v = vm.get_register_value(r1.into());
	let r2v = vm.get_register_value(r2.into());
	vm.eq = r1v == r2v;
	vm.ng = (r1v as i64) < (r2v as i64);
}
