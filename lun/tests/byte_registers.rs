use lun::prelude::*;

#[test]
fn byte_registers() {
    let mut vm = Vm::default();
    vm.set_register_value(x, 0x0706050403020100);

    for i in 0..8 {
        let xbi = VmByteRegister::new(x, i);
        assert_eq!(i, vm.get_partial_register_value(xbi));
    }
}
