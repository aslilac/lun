use super::super::iprelude::*;

pub fn b_i(vm: &mut Vm, i: u8) {
    vm.disp.push(i);
}

pub fn b_r(vm: &mut Vm, r1: VmByteRegister) {
    let r1v = vm.get_partial_register_value(r1) as u8;
    vm.disp.push(r1v);
}

pub fn w_r(vm: &mut Vm, r1: VmRegister) {
    let r1v = vm.get_register_value(r1);
    for byte in r1v.to_le_bytes() {
        if byte == 0 {
            break;
        }

        vm.disp.push(byte);
    }
}
