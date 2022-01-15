use super::super::iprelude::*;

pub fn r(vm: &mut Vm, r1: VmRegister) {
    let sp_update = |prev: u64| prev.checked_add(1).ok_or(StackUnderflow);

    if let Ok(sp) = vm.update_register_value(s, sp_update) {
        let sv = vm.mem[(sp - 1) as usize];
        vm.set_register_value(r1, sv);
    }
}
