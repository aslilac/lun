mod add;
mod div;
mod mul;

use super::iprelude::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArithmeticInstruction {
	add_i8_rir(VmByteRegister, i8, VmByteRegister),
	add_i8_rrr(VmByteRegister, VmByteRegister, VmByteRegister),
	add_u8_rir(VmByteRegister, u8, VmByteRegister),
	add_u8_rrr(VmByteRegister, VmByteRegister, VmByteRegister),
	add_i16_rir(VmQwordRegister, i16, VmQwordRegister),
	add_i16_rrr(VmQwordRegister, VmQwordRegister, VmQwordRegister),
	add_u16_rir(VmQwordRegister, u16, VmQwordRegister),
	add_u16_rrr(VmQwordRegister, VmQwordRegister, VmQwordRegister),
	add_f32_rir(VmHwordRegister, f32, VmHwordRegister),
	add_f32_rrr(VmHwordRegister, VmHwordRegister, VmHwordRegister),
	add_i32_rir(VmHwordRegister, i32, VmHwordRegister),
	add_i32_rrr(VmHwordRegister, VmHwordRegister, VmHwordRegister),
	add_u32_rir(VmHwordRegister, u32, VmHwordRegister),
	add_u32_rrr(VmHwordRegister, VmHwordRegister, VmHwordRegister),
	add_f64_rir(VmWordRegister, f64, VmWordRegister),
	add_f64_rrr(VmWordRegister, VmWordRegister, VmWordRegister),
	add_i64_rir(VmWordRegister, i64, VmWordRegister),
	add_i64_rrr(VmWordRegister, VmWordRegister, VmWordRegister),
	add_u64_rir(VmWordRegister, u64, VmWordRegister),
	add_u64_rrr(VmWordRegister, VmWordRegister, VmWordRegister),

	div_i64_rirr(VmWordRegister, i64, VmWordRegister, VmWordRegister),
	div_i64_rrrr(
		VmWordRegister,
		VmWordRegister,
		VmWordRegister,
		VmWordRegister,
	),

	mul_i64_rir(VmWordRegister, i64, VmWordRegister),
	mul_i64_rrr(VmWordRegister, VmWordRegister, VmWordRegister),
	mul_u64_rir(VmWordRegister, u64, VmWordRegister),
	mul_u64_rrr(VmWordRegister, VmWordRegister, VmWordRegister),
}

impl Instruction for ArithmeticInstruction {
	fn exec(self, vm: &mut Vm) {
		use ArithmeticInstruction::*;

		match self {
			add_i8_rir(r1, i, rr) => add::i8_rir(vm, r1, i, rr),
			add_i8_rrr(r1, r2, rr) => add::i8_rrr(vm, r1, r2, rr),
			add_u8_rir(r1, i, rr) => add::u8_rir(vm, r1, i, rr),
			add_u8_rrr(r1, r2, rr) => add::u8_rrr(vm, r1, r2, rr),
			add_i16_rir(r1, i, rr) => add::i16_rir(vm, r1, i, rr),
			add_i16_rrr(r1, r2, rr) => add::i16_rrr(vm, r1, r2, rr),
			add_u16_rir(r1, i, rr) => add::u16_rir(vm, r1, i, rr),
			add_u16_rrr(r1, r2, rr) => add::u16_rrr(vm, r1, r2, rr),
			add_f32_rir(r1, i, rr) => add::f32_rir(vm, r1, i, rr),
			add_f32_rrr(r1, r2, rr) => add::f32_rrr(vm, r1, r2, rr),
			add_i32_rir(r1, i, rr) => add::i32_rir(vm, r1, i, rr),
			add_i32_rrr(r1, r2, rr) => add::i32_rrr(vm, r1, r2, rr),
			add_u32_rir(r1, i, rr) => add::u32_rir(vm, r1, i, rr),
			add_u32_rrr(r1, r2, rr) => add::u32_rrr(vm, r1, r2, rr),
			add_f64_rir(r1, i, rr) => add::f64_rir(vm, r1, i, rr),
			add_f64_rrr(r1, r2, rr) => add::f64_rrr(vm, r1, r2, rr),
			add_i64_rir(r1, i, rr) => add::i64_rir(vm, r1, i, rr),
			add_i64_rrr(r1, r2, rr) => add::i64_rrr(vm, r1, r2, rr),
			add_u64_rir(r1, i, rr) => add::u64_rir(vm, r1, i, rr),
			add_u64_rrr(r1, r2, rr) => add::u64_rrr(vm, r1, r2, rr),

			div_i64_rirr(r1, i, rr, rm) => div::i64_rirr(vm, r1, i, rr, rm),
			div_i64_rrrr(r1, r2, rr, rm) => div::i64_rrrr(vm, r1, r2, rr, rm),

			mul_i64_rir(r1, i, rr) => mul::i64_rir(vm, r1, i, rr),
			mul_i64_rrr(r1, r2, rr) => mul::i64_rrr(vm, r1, r2, rr),
			mul_u64_rir(r1, i, rr) => mul::u64_rir(vm, r1, i, rr),
			mul_u64_rrr(r1, r2, rr) => mul::u64_rrr(vm, r1, r2, rr),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{super::iprelude::*, ArithmeticInstruction::*};

	#[test]
	fn add_i8() {
		let mut vm = Vm::default();

		vm.set_register_value(reg::xb1, 1);

		vm.exec(add_i8_rrr(
			reg::xb0.into(),
			reg::xb1.into(),
			reg::xb0.into(),
		));
		assert_eq!(vm.get_register_value(reg::xb0), 1);
		vm.exec(add_i8_rrr(
			reg::xb0.into(),
			reg::xb1.into(),
			reg::xb0.into(),
		));
		assert_eq!(vm.get_register_value(reg::xb0), 2);

		vm.set_register_value(reg::xb0, i8::MAX as u64);

		vm.exec(add_i8_rrr(
			reg::xb0.into(),
			reg::xb1.into(),
			reg::xb0.into(),
		));
		assert_eq!(vm.get_register_value(reg::xb0), 0x80);
		assert_eq!(vm.ov, true);
		vm.exec(add_i8_rrr(
			reg::xb0.into(),
			reg::xb1.into(),
			reg::xb0.into(),
		));
		assert_eq!(vm.get_register_value(reg::xb0), 0x81);
		assert_eq!(vm.ov, false);
	}

	#[test]
	fn add_u8() {
		let mut vm = Vm::default();

		vm.set_register_value(reg::xb1, 1);

		vm.exec(add_u8_rrr(
			reg::xb0.into(),
			reg::xb1.into(),
			reg::xb0.into(),
		));
		assert_eq!(vm.get_register_value(reg::xb0), 1);
		vm.exec(add_u8_rrr(
			reg::xb0.into(),
			reg::xb1.into(),
			reg::xb0.into(),
		));
		assert_eq!(vm.get_register_value(reg::xb0), 2);

		vm.set_register_value(reg::xb0, u8::MAX as u64);

		vm.exec(add_u8_rrr(
			reg::xb0.into(),
			reg::xb1.into(),
			reg::xb0.into(),
		));
		assert_eq!(vm.get_register_value(reg::xb0), 0x00);
		assert_eq!(vm.ov, true);
		vm.exec(add_u8_rrr(
			reg::xb0.into(),
			reg::xb1.into(),
			reg::xb0.into(),
		));
		assert_eq!(vm.get_register_value(reg::xb0), 0x01);
		assert_eq!(vm.ov, false);
	}

	#[test]
	fn add_f32() {
		let mut vm = Vm::default();

		vm.exec(add_f32_rir(reg::xh0.into(), 1.0, reg::xh0.into()));
		assert_eq!(vm.get_register_value(reg::xh0), 0x3f800000);
	}

	#[test]
	fn add_f64() {
		let mut vm = Vm::default();

		vm.exec(add_f64_rir(reg::x.into(), 1.0, reg::x.into()));
		vm.exec(add_f64_rir(reg::y.into(), 5.47, reg::y.into()));
		vm.exec(add_f64_rrr(reg::x.into(), reg::y.into(), reg::x.into()));
		vm.exec(add_f64_rrr(reg::x.into(), reg::y.into(), reg::a.into()));
		assert_eq!(vm.get_register_value(reg::a), 0x4027e147ae147ae1);
	}

	#[test]
	fn add_i64() {
		let mut vm = Vm::default();

		vm.exec(add_i64_rir(reg::x.into(), 2, reg::x.into()));
		assert_eq!(vm.get_register_value(reg::x), 2);
		vm.exec(add_i64_rrr(reg::x.into(), reg::x.into(), reg::x.into()));
		assert_eq!(vm.get_register_value(reg::x), 4);

		vm.exec(add_i64_rir(reg::x.into(), -8, reg::x.into()));
		assert_eq!(vm.get_register_value(reg::x) as i64, -4);
	}

	#[test]
	fn add_u64() {
		let mut vm = Vm::default();

		vm.exec(add_u64_rir(reg::x.into(), 2, reg::x.into()));
		assert_eq!(vm.get_register_value(reg::x), 2);
		vm.exec(add_u64_rrr(reg::x.into(), reg::x.into(), reg::x.into()));
		assert_eq!(vm.get_register_value(reg::x), 4);
	}

	#[test]
	fn mul_i64() {
		let mut vm = Vm::default();

		vm.set_register_value(reg::x, 5);

		vm.exec(mul_i64_rir(reg::x.into(), -1, reg::x.into()));
		assert_eq!(vm.get_register_value(reg::x) as i64, -5);

		vm.exec(mul_i64_rir(reg::x.into(), 2, reg::x.into()));
		assert_eq!(vm.get_register_value(reg::x) as i64, -10);

		vm.set_register_value(reg::x, u64::MAX); // should be the same as -1
		vm.set_register_value(reg::y, 5);

		vm.exec(mul_i64_rrr(reg::x.into(), reg::y.into(), reg::x.into()));
		assert_eq!(vm.get_register_value(reg::x) as i64, -5);

		vm.exec(mul_i64_rrr(reg::x.into(), reg::x.into(), reg::x.into()));
		assert_eq!(vm.get_register_value(reg::x) as i64, 25);
	}

	#[test]
	fn mul_u64() {
		let mut vm = Vm::default();

		vm.set_register_value(reg::x, 2);

		vm.exec(mul_u64_rrr(reg::x.into(), reg::x.into(), reg::x.into()));
		assert_eq!(vm.get_register_value(reg::x), 4);
	}
}
