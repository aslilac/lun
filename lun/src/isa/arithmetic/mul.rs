use super::super::iprelude::*;

pub fn i64_rir(vm: &mut Vm, r1: VmNativeRegister, i: i64, rr: VmNativeRegister) {
	let r1v = vm.get_register_value(r1) as i64;
	let (result, overflow) = r1v.overflowing_mul(i);
	vm.set_register_value(rr, result as u64);
	vm.ov = overflow;
}

pub fn i64_rrr(vm: &mut Vm, r1: VmNativeRegister, r2: VmNativeRegister, rr: VmNativeRegister) {
	let r1v = vm.get_register_value(r1) as i64;
	let r2v = vm.get_register_value(r2) as i64;
	let (result, overflow) = r1v.overflowing_mul(r2v);
	vm.set_register_value(rr, result as u64);
	vm.ov = overflow;
}

pub fn u64_rir(vm: &mut Vm, r1: VmNativeRegister, i: u64, rr: VmNativeRegister) {
	let r1v = vm.get_register_value(r1);
	let (result, overflow) = r1v.overflowing_mul(i);
	vm.set_register_value(rr, result);
	vm.ov = overflow;
}

pub fn u64_rrr(vm: &mut Vm, r1: VmNativeRegister, r2: VmNativeRegister, rr: VmNativeRegister) {
	let r1v = vm.get_register_value(r1);
	let r2v = vm.get_register_value(r2);
	let (result, overflow) = r1v.overflowing_mul(r2v);
	vm.set_register_value(rr, result);
	vm.ov = overflow;
}
