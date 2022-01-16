use super::super::iprelude::*;

pub fn i64_rirr(
    vm: &mut Vm,
    r1: VmRegister,
    i: i64,
    rro: Option<VmRegister>,
    rmo: Option<VmRegister>,
) {
    let r1v = vm.get_register_value(r1) as i64;
    let (quotient, m) = (r1v / i, r1v % i);

    if let Some(rr) = rro {
        vm.set_register_value(rr, quotient as u64);
    }

    if let Some(rm) = rmo {
        vm.set_register_value(rm, m as u64);
    }
}

pub fn i64_rrrr(
    vm: &mut Vm,
    r1: VmRegister,
    r2: VmRegister,
    rro: Option<VmRegister>,
    rmo: Option<VmRegister>,
) {
    let r1v = vm.get_register_value(r1) as i64;
    let r2v = vm.get_register_value(r2) as i64;
    let (quotient, m) = (r1v / r2v, r1v % r2v);

    if let Some(rr) = rro {
        vm.set_register_value(rr, quotient as u64);
    }

    if let Some(rm) = rmo {
        vm.set_register_value(rm, m as u64);
    }
}
