mod add;
mod div;
mod mul;

use super::iprelude::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArithmeticInstruction {
    add_i8_ri(VmByteRegister, i8),
    add_i8_rr(VmByteRegister, VmByteRegister),
    add_i8_rir(VmByteRegister, i8, VmByteRegister),
    add_i8_rrr(VmByteRegister, VmByteRegister, VmByteRegister),
    add_u8_ri(VmByteRegister, u8),
    add_u8_rr(VmByteRegister, VmByteRegister),
    add_u8_rir(VmByteRegister, u8, VmByteRegister),
    add_u8_rrr(VmByteRegister, VmByteRegister, VmByteRegister),
    add_i16_ri(VmQwordRegister, i16),
    add_i16_rr(VmQwordRegister, VmQwordRegister),
    add_i16_rir(VmQwordRegister, i16, VmQwordRegister),
    add_i16_rrr(VmQwordRegister, VmQwordRegister, VmQwordRegister),
    add_u16_ri(VmQwordRegister, u16),
    add_u16_rr(VmQwordRegister, VmQwordRegister),
    add_u16_rir(VmQwordRegister, u16, VmQwordRegister),
    add_u16_rrr(VmQwordRegister, VmQwordRegister, VmQwordRegister),
    add_f32_ri(VmHwordRegister, f32),
    add_f32_rr(VmHwordRegister, VmHwordRegister),
    add_f32_rir(VmHwordRegister, f32, VmHwordRegister),
    add_f32_rrr(VmHwordRegister, VmHwordRegister, VmHwordRegister),
    add_i32_ri(VmHwordRegister, i32),
    add_i32_rr(VmHwordRegister, VmHwordRegister),
    add_i32_rir(VmHwordRegister, i32, VmHwordRegister),
    add_i32_rrr(VmHwordRegister, VmHwordRegister, VmHwordRegister),
    add_u32_ri(VmHwordRegister, u32),
    add_u32_rr(VmHwordRegister, VmHwordRegister),
    add_u32_rir(VmHwordRegister, u32, VmHwordRegister),
    add_u32_rrr(VmHwordRegister, VmHwordRegister, VmHwordRegister),
    add_f64_ri(VmRegister, f64),
    add_f64_rr(VmRegister, VmRegister),
    add_f64_rir(VmRegister, f64, VmRegister),
    add_f64_rrr(VmRegister, VmRegister, VmRegister),
    add_i64_ri(VmRegister, i64),
    add_i64_rr(VmRegister, VmRegister),
    add_i64_rir(VmRegister, i64, VmRegister),
    add_i64_rrr(VmRegister, VmRegister, VmRegister),
    add_u64_ri(VmRegister, u64),
    add_u64_rr(VmRegister, VmRegister),
    add_u64_rir(VmRegister, u64, VmRegister),
    add_u64_rrr(VmRegister, VmRegister, VmRegister),

    div_q_i64_rr(VmRegister, VmRegister),
    div_q_i64_rrr(VmRegister, VmRegister, VmRegister),
    div_m_i64_rr(VmRegister, VmRegister),
    div_m_i64_rrr(VmRegister, VmRegister, VmRegister),
    div_i64_rr(VmRegister, VmRegister),
    div_i64_rrr(VmRegister, VmRegister, VmRegister),
    div_i64_rrrr(VmRegister, VmRegister, VmRegister, VmRegister),

    mul_i64_ri(VmRegister, i64),
    mul_i64_rr(VmRegister, VmRegister),
    mul_i64_rir(VmRegister, i64, VmRegister),
    mul_i64_rrr(VmRegister, VmRegister, VmRegister),
    mul_u64_ri(VmRegister, u64),
    mul_u64_rr(VmRegister, VmRegister),
    mul_u64_rir(VmRegister, u64, VmRegister),
    mul_u64_rrr(VmRegister, VmRegister, VmRegister),
}

impl Instruction for ArithmeticInstruction {
    fn exec(self, vm: &mut Vm) {
        use ArithmeticInstruction::*;

        match self {
            add_i8_ri(r1, i) => add::i8_rir(vm, r1, i, r1),
            add_i8_rr(r1, r2) => add::i8_rrr(vm, r1, r2, r1),
            add_i8_rir(r1, i, rr) => add::i8_rir(vm, r1, i, rr),
            add_i8_rrr(r1, r2, rr) => add::i8_rrr(vm, r1, r2, rr),
            add_u8_ri(r1, i) => add::u8_rir(vm, r1, i, r1),
            add_u8_rr(r1, r2) => add::u8_rrr(vm, r1, r2, r1),
            add_u8_rir(r1, i, rr) => add::u8_rir(vm, r1, i, rr),
            add_u8_rrr(r1, r2, rr) => add::u8_rrr(vm, r1, r2, rr),
            add_i16_ri(r1, i) => add::i16_rir(vm, r1, i, r1),
            add_i16_rr(r1, r2) => add::i16_rrr(vm, r1, r2, r1),
            add_i16_rir(r1, i, rr) => add::i16_rir(vm, r1, i, rr),
            add_i16_rrr(r1, r2, rr) => add::i16_rrr(vm, r1, r2, rr),
            add_u16_ri(r1, i) => add::u16_rir(vm, r1, i, r1),
            add_u16_rr(r1, r2) => add::u16_rrr(vm, r1, r2, r1),
            add_u16_rir(r1, i, rr) => add::u16_rir(vm, r1, i, rr),
            add_u16_rrr(r1, r2, rr) => add::u16_rrr(vm, r1, r2, rr),
            add_f32_ri(r1, i) => add::f32_rir(vm, r1, i, r1),
            add_f32_rr(r1, r2) => add::f32_rrr(vm, r1, r2, r1),
            add_f32_rir(r1, i, rr) => add::f32_rir(vm, r1, i, rr),
            add_f32_rrr(r1, r2, rr) => add::f32_rrr(vm, r1, r2, rr),
            add_i32_ri(r1, i) => add::i32_rir(vm, r1, i, r1),
            add_i32_rr(r1, r2) => add::i32_rrr(vm, r1, r2, r1),
            add_i32_rir(r1, i, rr) => add::i32_rir(vm, r1, i, rr),
            add_i32_rrr(r1, r2, rr) => add::i32_rrr(vm, r1, r2, rr),
            add_u32_ri(r1, i) => add::u32_rir(vm, r1, i, r1),
            add_u32_rr(r1, r2) => add::u32_rrr(vm, r1, r2, r1),
            add_u32_rir(r1, i, rr) => add::u32_rir(vm, r1, i, rr),
            add_u32_rrr(r1, r2, rr) => add::u32_rrr(vm, r1, r2, rr),
            add_f64_ri(r1, i) => add::f64_rir(vm, r1, i, r1),
            add_f64_rr(r1, r2) => add::f64_rrr(vm, r1, r2, r1),
            add_f64_rir(r1, i, rr) => add::f64_rir(vm, r1, i, rr),
            add_f64_rrr(r1, r2, rr) => add::f64_rrr(vm, r1, r2, rr),
            add_i64_ri(r1, i) => add::i64_rir(vm, r1, i, r1),
            add_i64_rr(r1, r2) => add::i64_rrr(vm, r1, r2, r1),
            add_i64_rir(r1, i, rr) => add::i64_rir(vm, r1, i, rr),
            add_i64_rrr(r1, r2, rr) => add::i64_rrr(vm, r1, r2, rr),
            add_u64_ri(r1, i) => add::u64_rir(vm, r1, i, r1),
            add_u64_rr(r1, r2) => add::u64_rrr(vm, r1, r2, r1),
            add_u64_rir(r1, i, rr) => add::u64_rir(vm, r1, i, rr),
            add_u64_rrr(r1, r2, rr) => add::u64_rrr(vm, r1, r2, rr),

            div_q_i64_rr(r1, r2) => div::i64_rrrr(vm, r1, r2, Some(r1), None),
            div_q_i64_rrr(r1, r2, rr) => div::i64_rrrr(vm, r1, r2, Some(rr), None),
            div_m_i64_rr(r1, r2) => div::i64_rrrr(vm, r1, r2, None, Some(r1)),
            div_m_i64_rrr(r1, r2, rr) => div::i64_rrrr(vm, r1, r2, None, Some(rr)),
            div_i64_rr(r1, r2) => div::i64_rrrr(vm, r1, r2, Some(r1), Some(r)),
            div_i64_rrr(r1, r2, rr) => div::i64_rrrr(vm, r1, r2, Some(rr), Some(r)),
            div_i64_rrrr(r1, r2, rr, rm) => div::i64_rrrr(vm, r1, r2, Some(rr), Some(rm)),

            mul_i64_ri(r1, i) => mul::i64_rir(vm, r1, i, r1),
            mul_i64_rr(r1, r2) => mul::i64_rrr(vm, r1, r2, r1),
            mul_i64_rir(r1, i, rr) => mul::i64_rir(vm, r1, i, rr),
            mul_i64_rrr(r1, r2, rr) => mul::i64_rrr(vm, r1, r2, rr),
            mul_u64_ri(r1, i) => mul::u64_rir(vm, r1, i, r1),
            mul_u64_rr(r1, r2) => mul::u64_rrr(vm, r1, r2, r1),
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

        let xb0 = VmByteRegister::new(x, 0);
        let xb1 = VmByteRegister::new(x, 1);

        vm.set_partial_register_value(xb1, 1);

        vm.exec(add_i8_rr(xb0, xb1));
        assert_eq!(vm.get_partial_register_value(xb0), 1);
        vm.exec(add_i8_rr(xb0, xb1));
        assert_eq!(vm.get_partial_register_value(xb0), 2);

        vm.set_partial_register_value(xb0, i8::MAX as u64);

        vm.exec(add_i8_rr(xb0, xb1));
        assert_eq!(vm.get_partial_register_value(xb0), 0x80);
        assert_eq!(vm.ov, true);
        vm.exec(add_i8_rr(xb0, xb1));
        assert_eq!(vm.get_partial_register_value(xb0), 0x81);
        assert_eq!(vm.ov, false);
    }

    #[test]
    fn add_u8() {
        let mut vm = Vm::default();

        let xb0 = VmByteRegister::new(x, 0);
        let xb1 = VmByteRegister::new(x, 1);

        vm.set_partial_register_value(xb1, 1);

        vm.exec(add_u8_rr(xb0, xb1));
        assert_eq!(vm.get_partial_register_value(xb0), 1);
        vm.exec(add_u8_rr(xb0, xb1));
        assert_eq!(vm.get_partial_register_value(xb0), 2);

        vm.set_partial_register_value(xb0, u8::MAX as u64);

        vm.exec(add_u8_rr(xb0, xb1));
        assert_eq!(vm.get_partial_register_value(xb0), 0x00);
        assert_eq!(vm.ov, true);
        vm.exec(add_u8_rr(xb0, xb1));
        assert_eq!(vm.get_partial_register_value(xb0), 0x01);
        assert_eq!(vm.ov, false);
    }

    #[test]
    fn add_f32() {
        let mut vm = Vm::default();
        let xh0 = VmHwordRegister::new(x, 0);

        vm.exec(add_f32_ri(xh0, 1.0));
        assert_eq!(vm.get_partial_register_value(xh0), 0x3f800000);
    }

    #[test]
    fn add_f64() {
        let mut vm = Vm::default();

        vm.exec(add_f64_ri(x, 1.0));
        vm.exec(add_f64_ri(y, 5.47));
        vm.exec(add_f64_rr(x, y));
        vm.exec(add_f64_rrr(x, y, a));
        assert_eq!(vm.get_register_value(a), 0x4027e147ae147ae1);
    }

    #[test]
    fn add_i64() {
        let mut vm = Vm::default();

        vm.exec(add_i64_ri(x, 2));
        assert_eq!(vm.get_register_value(x), 2);
        vm.exec(add_i64_rr(x, x));
        assert_eq!(vm.get_register_value(x), 4);

        vm.exec(add_i64_ri(x, -8));
        assert_eq!(vm.get_register_value(x) as i64, -4);
    }

    #[test]
    fn add_u64() {
        let mut vm = Vm::default();

        vm.exec(add_u64_ri(x, 2));
        assert_eq!(vm.get_register_value(x), 2);
        vm.exec(add_u64_rr(x, x));
        assert_eq!(vm.get_register_value(x), 4);
    }

    #[test]
    fn mul_i64() {
        let mut vm = Vm::default();

        vm.set_register_value(x, 5);

        vm.exec(mul_i64_ri(x, -1));
        assert_eq!(vm.get_register_value(x) as i64, -5);

        vm.exec(mul_i64_ri(x, 2));
        assert_eq!(vm.get_register_value(x) as i64, -10);

        vm.set_register_value(x, u64::MAX); // should be the same as -1
        vm.set_register_value(y, 5);

        vm.exec(mul_i64_rr(x, y));
        assert_eq!(vm.get_register_value(x) as i64, -5);

        vm.exec(mul_i64_rr(x, x));
        assert_eq!(vm.get_register_value(x) as i64, 25);
    }

    #[test]
    fn mul_u64() {
        let mut vm = Vm::default();

        vm.set_register_value(x, 2);

        vm.exec(mul_u64_rr(x, x));
        assert_eq!(vm.get_register_value(x), 4);
    }
}
