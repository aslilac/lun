use super::super::iprelude::*;
use std::io::Write;

pub fn b_i(vm: &mut Vm, i: u8) {
	// write!(vm.disp, i);
	let _ = vm.disp.write_all(&[i]);
}

pub fn b_r(vm: &mut Vm, r1: VmByteRegister) {
	let r1v = vm.get_register_value(r1.into()) as u8;
	let _ = vm.disp.write_all(&[r1v]);
}

pub fn w_r(vm: &mut Vm, r1: VmWordRegister) {
	let r1v = vm.get_register_value(r1.into());
	let _ = vm.disp.write(&r1v.to_le_bytes());
}
