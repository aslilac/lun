use crate::vm::{Vm, VmRegister};

pub fn u64_rir(vm: &mut Vm, r1: VmRegister, i: u64, rr: VmRegister) {
    let r1v = vm.get_register_value(r1);
    vm.set_register_value(rr, r1v.overflowing_mul(i).0);
}

pub fn u64_rrr(vm: &mut Vm, r1: VmRegister, r2: VmRegister, rr: VmRegister) {
    let r1v = vm.get_register_value(r1);
    let r2v = vm.get_register_value(r2);
    vm.set_register_value(rr, r1v.overflowing_mul(r2v).0);
}
