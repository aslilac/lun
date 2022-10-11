use super::super::iprelude::*;

pub fn i64_rirr(vm: &mut Vm, r1: VmWordRegister, i: i64, rr: VmWordRegister, rm: VmWordRegister) {
	let r1v = vm.get_register_value(r1.into()) as i64;
	let (quotient, m) = (r1v / i, r1v % i);
	vm.set_register_value(rr.into(), quotient as u64);
	vm.set_register_value(rm.into(), m as u64);
}

pub fn i64_rrrr(
	vm: &mut Vm,
	r1: VmWordRegister,
	r2: VmWordRegister,
	rr: VmWordRegister,
	rm: VmWordRegister,
) {
	let r1v = vm.get_register_value(r1.into()) as i64;
	let r2v = vm.get_register_value(r2.into()) as i64;
	let (quotient, m) = (r1v / r2v, r1v % r2v);
	vm.set_register_value(rr.into(), quotient as u64);
	vm.set_register_value(rm.into(), m as u64);
}
