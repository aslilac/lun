use super::super::iprelude::*;

pub fn i64_rir(vm: &mut Vm, r1: VmWordRegister, i: i64, rr: VmWordRegister) {
	let r1v = vm.get_register_value(r1.into()) as i64;
	let (result, overflow) = r1v.overflowing_mul(i);
	vm.set_register_value(rr.into(), result as u64);
	vm.ov = overflow;
}

pub fn i64_rrr(vm: &mut Vm, r1: VmWordRegister, r2: VmWordRegister, rr: VmWordRegister) {
	let r1v = vm.get_register_value(r1.into()) as i64;
	let r2v = vm.get_register_value(r2.into()) as i64;
	let (result, overflow) = r1v.overflowing_mul(r2v);
	vm.set_register_value(rr.into(), result as u64);
	vm.ov = overflow;
}

pub fn u64_rir(vm: &mut Vm, r1: VmWordRegister, i: u64, rr: VmWordRegister) {
	let r1v = vm.get_register_value(r1.into());
	let (result, overflow) = r1v.overflowing_mul(i);
	vm.set_register_value(rr.into(), result);
	vm.ov = overflow;
}

pub fn u64_rrr(vm: &mut Vm, r1: VmWordRegister, r2: VmWordRegister, rr: VmWordRegister) {
	let r1v = vm.get_register_value(r1.into());
	let r2v = vm.get_register_value(r2.into());
	let (result, overflow) = r1v.overflowing_mul(r2v);
	vm.set_register_value(rr.into(), result);
	vm.ov = overflow;
}
