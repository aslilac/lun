use super::super::iprelude::*;

pub fn t_rrr<T: VmRegister>(vm: &mut Vm, r1: T, r2: T, rr: T) {
	let r1v = vm.get_register_value(r1);
	let r2v = vm.get_register_value(r2);
	vm.set_register_value(rr, r1v & r2v);
}
