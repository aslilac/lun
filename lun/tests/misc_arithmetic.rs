use lun::prelude::*;

#[test]
fn random_arithmatic_test() {
	let mut vm = Vm::default();

	// Initialize to 0, use as a counter
	vm.exec(xor_t_r(reg::a));
	assert_eq!(0, vm.get_register_value(reg::a));

	// Initialize to 0, use as an accumulator
	vm.exec(xor_t_r(reg::b));

	// Initialize to 0, used to find fibbonacci numbers
	vm.exec(xor_t_r(reg::c));
	vm.exec(xor_t_r(reg::d));
	vm.exec(xor_t_r(reg::e));

	vm.exec(add_u64_rir(reg::c.into(), 1, reg::c.into())); // 0 + 1 = 1
	vm.exec(add_u64_rir(reg::d.into(), 1, reg::d.into())); // 0 + 1 = 1

	for _ in 0..10 {
		// Increment a
		vm.exec(add_u64_rir(reg::a.into(), 1, reg::a.into()));

		// b accumulates a
		vm.exec(add_u64_rrr(reg::b.into(), reg::a.into(), reg::b.into()));

		// [c, d, e] = [d, e, c + d]
		vm.exec(add_u64_rrr(reg::c.into(), reg::d.into(), reg::e.into()));
		vm.exec(add_u64_rir(reg::d.into(), 0, reg::c.into()));
		vm.exec(add_u64_rir(reg::e.into(), 0, reg::d.into()));

		println!(
			"[ a: {: <5}, b: {: <5}, c: {: <5}, d: {: <5}, e: {: <5} ]",
			vm.get_register_value(reg::a),
			vm.get_register_value(reg::b),
			vm.get_register_value(reg::c),
			vm.get_register_value(reg::d),
			vm.get_register_value(reg::e),
		);
	}

	assert_eq!(10, vm.get_register_value(reg::a));
	assert_eq!(55, vm.get_register_value(reg::b));
	assert_eq!(89, vm.get_register_value(reg::c));
	assert_eq!(144, vm.get_register_value(reg::d));
	assert_eq!(144, vm.get_register_value(reg::e));

	// Clear to 0 (short form)
	vm.exec(xor_t_r(reg::a));
	assert_eq!(0, vm.get_register_value(reg::a));

	// Make it not 0 again
	vm.exec(add_u64_rir(reg::a.into(), 10, reg::a.into()));
	assert_eq!(10, vm.get_register_value(reg::a));

	// Clear to 0 (long form)
	vm.exec(xor_t_rrr(reg::a.into(), reg::a.into(), reg::a.into()));
	assert_eq!(0, vm.get_register_value(reg::a));
}
