use super::super::iprelude::*;

pub fn x_i(vm: &mut Vm, i: i64) {
	let _ = vm.update_register_value(p, |prev| {
		let result = prev.checked_add_signed(i);

		if let Some(value) = result {
			Ok(value)
		} else {
			println!(
				"encountered overflow while branching:\n {} + {} {}",
				prev, i, "resulted in unacceptable program counter overflow"
			);
			Err(BrOverflow)
		}
	});
}
