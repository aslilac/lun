use super::super::iprelude::*;

pub fn i(vm: &mut Vm, i: u8) {
    println!("{i:x}");
    vm.dsp_buf.push(i);
}

pub fn r(vm: &mut Vm, r1: VmRegister) {
    let r1v = vm.get_register_value(r1);
    for byte in r1v.to_le_bytes() {
        if byte == 0 {
            break;
        }
        vm.dsp_buf.push(byte);
    }
}
