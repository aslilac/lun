use crate::vm::{Vm, VmRegister};

pub fn i64_rir(vm: &mut Vm, r1: VmRegister, i: i64, rr: VmRegister) {
    let r1v = vm.get_register_value(r1)as i64;
    let (result, overflow) = r1v.overflowing_add(i);
    vm.set_register_value(rr, result as u64);
}

pub fn i64_rrr(vm: &mut Vm, r1: VmRegister, r2: VmRegister, rr: VmRegister) {
    let r1v = vm.get_register_value(r1) as i64;
    let r2v = vm.get_register_value(r2) as i64;
    let (result, overflow) = r1v.overflowing_add(r2v);
    vm.set_register_value(rr, result as u64);
}
