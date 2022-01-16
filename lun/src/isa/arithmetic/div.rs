use super::super::iprelude::*;

pub fn i64_rirr(
    vm: &mut Vm,
    r1: VmNativeRegister,
    i: i64,
    rr: VmNativeRegister,
    rm: VmNativeRegister,
) {
    let r1v = vm.get_register_value(r1) as i64;
    let (quotient, m) = (r1v / i, r1v % i);
    vm.set_register_value(rr, quotient as u64);
    vm.set_register_value(rm, m as u64);
}

pub fn i64_rrrr(
    vm: &mut Vm,
    r1: VmNativeRegister,
    r2: VmNativeRegister,
    rr: VmNativeRegister,
    rm: VmNativeRegister,
) {
    let r1v = vm.get_register_value(r1) as i64;
    let r2v = vm.get_register_value(r2) as i64;
    let (quotient, m) = (r1v / r2v, r1v % r2v);
    vm.set_register_value(rr, quotient as u64);
    vm.set_register_value(rm, m as u64);
}
