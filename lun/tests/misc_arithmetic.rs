use lun::prelude::*;

#[test]
fn random_arithmatic_test() {
	let mut vm = Vm::default();

	// Initialize to 0, use as a counter
	vm.exec(xor_w_r(a));
	assert_eq!(0, vm.get_register_value(a));

	// Initialize to 0, use as an accumulator
	vm.exec(xor_w_r(b));

	// Initialize to 0, used to find fibbonacci numbers
	vm.exec(xor_w_r(c));
	vm.exec(xor_w_r(d));
	vm.exec(xor_w_r(e));

	vm.exec(add_u64_rir(c, 1, c)); // 0 + 1 = 1
	vm.exec(add_u64_rir(d, 1, d)); // 0 + 1 = 1

	for _ in 0..10 {
		// Increment a
		vm.exec(add_u64_rir(a, 1, a));

		// b accumulates a
		vm.exec(add_u64_rrr(b, a, b));

		// [c, d, e] = [d, e, c + d]
		vm.exec(add_u64_rrr(c, d, e));
		vm.exec(add_u64_rir(d, 0, c));
		vm.exec(add_u64_rir(e, 0, d));

		println!(
			"[ a: {: <5}, b: {: <5}, c: {: <5}, d: {: <5}, e: {: <5} ]",
			vm.get_register_value(a),
			vm.get_register_value(b),
			vm.get_register_value(c),
			vm.get_register_value(d),
			vm.get_register_value(e),
		);
	}

	assert_eq!(10, vm.get_register_value(a));
	assert_eq!(55, vm.get_register_value(b));
	assert_eq!(89, vm.get_register_value(c));
	assert_eq!(144, vm.get_register_value(d));
	assert_eq!(144, vm.get_register_value(e));

	// Clear to 0 (short form)
	vm.exec(xor_w_r(a));
	assert_eq!(0, vm.get_register_value(a));

	// Make it not 0 again
	vm.exec(add_u64_rir(a, 10, a));
	assert_eq!(10, vm.get_register_value(a));

	// Clear to 0 (long form)
	vm.exec(xor_w_rrr(a, a, a));
	assert_eq!(0, vm.get_register_value(a));
}
