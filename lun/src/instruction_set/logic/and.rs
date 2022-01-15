use super::super::iprelude::*;

pub fn rrr(vm: &mut Vm, r1: VmRegister, r2: VmRegister, rr: VmRegister) {
    let r1v = vm.get_register_value(r1);
    let r2v = vm.get_register_value(r2);
    vm.set_register_value(rr, r1v & r2v);
}
