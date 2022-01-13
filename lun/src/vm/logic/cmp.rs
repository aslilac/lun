use crate::vm::{Vm, VmRegister};

pub fn r(vm: &mut Vm, r1: VmRegister) {
    let r1v = vm.get_register_value(r1);
    vm.eq = r1v == 0;
    vm.ng = (r1v & (1 << 63)) > 0;
    vm.ov = false;
}

pub fn rr(vm: &mut Vm, r1: VmRegister, r2: VmRegister) {
    let r1v = vm.get_register_value(r1);
    let r2v = vm.get_register_value(r2);
    vm.eq = r1v == r2v;
    vm.ng = (r1v as i64) < (r2v as i64);
}
