use super::super::iprelude::*;

pub fn w_r(vm: &mut Vm, r1: VmNativeRegister) {
    let sp_update = |prev: u64| prev.checked_sub(1).ok_or(StackOverflow);

    if let Ok(sp) = vm.update_register_value(s, sp_update) {
        let r1v = vm.get_register_value(r1);
        vm.mem[sp as usize] = r1v;
    }
}
