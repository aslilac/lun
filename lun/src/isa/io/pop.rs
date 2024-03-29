use super::super::iprelude::*;

pub fn w_r(vm: &mut Vm, r1: VmWordRegister) {
	let sp_update = |prev: u64| prev.checked_add(1).ok_or(StackUnderflow);

	if let Ok(sp) = vm.update_register_value(reg::s, sp_update) {
		let sv = vm.mem[(sp - 1) as usize];
		vm.set_register_value(r1.into(), sv);
	}
}
