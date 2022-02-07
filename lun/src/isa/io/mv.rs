use super::super::iprelude::*;

pub fn t_rr<T: VmRegister>(vm: &mut Vm, r1: T, r2: T) {
    let r1v = vm.get_register_value(r1);
    let r2v = vm.get_register_value(r2);

    vm.set_register_value(r1, r2v);
    vm.set_register_value(r2, r1v);
}
