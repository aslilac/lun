use super::super::iprelude::*;

pub fn t_rr(vm: &mut Vm, r1: VmRegister, rr: VmRegister) {
	let r1v = vm.get_register_value(r1);

	vm.set_register_value(rr, r1v);
}
