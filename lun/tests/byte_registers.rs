use lun::prelude::*;

#[test]
fn byte_registers() {
    let mut vm = Vm::default();

    vm.set_register_value(x, 0x0706050403020100);
    println!(
        "xb0|xu8_0: {}",
        vm.get_partial_register_value(VmByteRegister::new(x, 0))
    );
    println!(
        "xb1|xu8_1: {}",
        vm.get_partial_register_value(VmByteRegister::new(x, 1))
    );
    println!(
        "xb2|xu8_2: {}",
        vm.get_partial_register_value(VmByteRegister::new(x, 2))
    );
    println!(
        "xb3|xu8_3: {}",
        vm.get_partial_register_value(VmByteRegister::new(x, 3))
    );
    println!(
        "xb4|xu8_4: {}",
        vm.get_partial_register_value(VmByteRegister::new(x, 4))
    );
    println!(
        "xb5|xu8_5: {}",
        vm.get_partial_register_value(VmByteRegister::new(x, 5))
    );
    println!(
        "xb6|xu8_6: {}",
        vm.get_partial_register_value(VmByteRegister::new(x, 6))
    );
    println!(
        "xb7|xu8_7: {}",
        vm.get_partial_register_value(VmByteRegister::new(x, 7))
    );
}
