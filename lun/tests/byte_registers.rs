use lun::prelude::*;

#[test]
fn byte_registers() {
	use VmNativeRegister::*;

	let mut vm = Vm::default();
	vm.set_register_value(reg::x, 0x0706050403020100);

	for i in 0..8 {
		let xbi = VmRegister::byte(X, i);
		assert_eq!(i, vm.get_register_value(xbi));
	}
}
