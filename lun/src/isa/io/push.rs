use super::super::iprelude::*;

pub fn w_r(vm: &mut Vm, r1: VmWordRegister) {
	let sp_update = |prev: u64| prev.checked_sub(1).ok_or(StackOverflow);

	if let Ok(sp) = vm.update_register_value(reg::s, sp_update) {
		let r1v = vm.get_register_value(r1.into());
		vm.mem[sp as usize] = r1v;
	}
}
