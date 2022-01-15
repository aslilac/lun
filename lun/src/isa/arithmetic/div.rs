use super::super::iprelude::*;

pub fn i64_rrrr(
    vm: &mut Vm,
    r1: VmRegister,
    r2: VmRegister,
    rro: Option<VmRegister>,
    rmo: Option<VmRegister>,
) {
    let r1v = vm.get_register_value(r1);
    let r2v = vm.get_register_value(r2);
    let (quotient, m) = (r1v / r2v, r1v % r2v);

    if let Some(rr) = rro {
        vm.set_register_value(rr, quotient);
    }

    if let Some(rm) = rmo {
        vm.set_register_value(rm, m);
    }
}
