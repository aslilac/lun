use super::super::iprelude::*;

pub fn i8_rir(vm: &mut Vm, r1: VmByteRegister, i: i8, rr: VmByteRegister) {
    let r1v = vm.get_partial_register_value(r1) as i8;
    let (result, overflow) = r1v.overflowing_add(i);
    vm.set_partial_register_value(rr, result as u64);
    vm.ov = overflow;
}

pub fn i8_rrr(vm: &mut Vm, r1: VmByteRegister, r2: VmByteRegister, rr: VmByteRegister) {
    let r1v = vm.get_partial_register_value(r1) as i8;
    let r2v = vm.get_partial_register_value(r2) as i8;
    let (result, overflow) = r1v.overflowing_add(r2v);
    vm.set_partial_register_value(rr, result as u64);
    vm.ov = overflow;
}

pub fn u8_rir(vm: &mut Vm, r1: VmByteRegister, i: u8, rr: VmByteRegister) {
    let r1v = vm.get_partial_register_value(r1) as u8;
    let (result, overflow) = r1v.overflowing_add(i);
    vm.set_partial_register_value(rr, result as u64);
    vm.ov = overflow;
}

pub fn u8_rrr(vm: &mut Vm, r1: VmByteRegister, r2: VmByteRegister, rr: VmByteRegister) {
    let r1v = vm.get_partial_register_value(r1) as u8;
    let r2v = vm.get_partial_register_value(r2) as u8;
    let (result, overflow) = r1v.overflowing_add(r2v);
    vm.set_partial_register_value(rr, result as u64);
    vm.ov = overflow;
}

pub fn i16_rir(vm: &mut Vm, r1: VmQwordRegister, i: i16, rr: VmQwordRegister) {
    let r1v = vm.get_partial_register_value(r1) as i16;
    let (result, overflow) = r1v.overflowing_add(i);
    vm.set_partial_register_value(rr, result as u64);
    vm.ov = overflow;
}

pub fn i16_rrr(vm: &mut Vm, r1: VmQwordRegister, r2: VmQwordRegister, rr: VmQwordRegister) {
    let r1v = vm.get_partial_register_value(r1) as i16;
    let r2v = vm.get_partial_register_value(r2) as i16;
    let (result, overflow) = r1v.overflowing_add(r2v);
    vm.set_partial_register_value(rr, result as u64);
    vm.ov = overflow;
}

pub fn u16_rir(vm: &mut Vm, r1: VmQwordRegister, i: u16, rr: VmQwordRegister) {
    let r1v = vm.get_partial_register_value(r1) as u16;
    let (result, overflow) = r1v.overflowing_add(i);
    vm.set_partial_register_value(rr, result as u64);
    vm.ov = overflow;
}

pub fn u16_rrr(vm: &mut Vm, r1: VmQwordRegister, r2: VmQwordRegister, rr: VmQwordRegister) {
    let r1v = vm.get_partial_register_value(r1) as u16;
    let r2v = vm.get_partial_register_value(r2) as u16;
    let (result, overflow) = r1v.overflowing_add(r2v);
    vm.set_partial_register_value(rr, result as u64);
    vm.ov = overflow;
}

pub fn f32_rir(vm: &mut Vm, r1: VmHwordRegister, i: f32, rr: VmHwordRegister) {
    let r1v = f32::from_bits(vm.get_partial_register_value(r1) as u32);
    let result = r1v + i;
    vm.set_partial_register_value(rr, result.to_bits() as u64);
    vm.ov = false;
}

pub fn f32_rrr(vm: &mut Vm, r1: VmHwordRegister, r2: VmHwordRegister, rr: VmHwordRegister) {
    let r1v = f32::from_bits(vm.get_partial_register_value(r1) as u32);
    let r2v = f32::from_bits(vm.get_partial_register_value(r2) as u32);
    let result = r1v + r2v;
    vm.set_partial_register_value(rr, result.to_bits() as u64);
    vm.ov = false;
}

pub fn i32_rir(vm: &mut Vm, r1: VmHwordRegister, i: i32, rr: VmHwordRegister) {
    let r1v = vm.get_partial_register_value(r1) as i32;
    let (result, overflow) = r1v.overflowing_add(i);
    vm.set_partial_register_value(rr, result as u64);
    vm.ov = overflow;
}

pub fn i32_rrr(vm: &mut Vm, r1: VmHwordRegister, r2: VmHwordRegister, rr: VmHwordRegister) {
    let r1v = vm.get_partial_register_value(r1) as i32;
    let r2v = vm.get_partial_register_value(r2) as i32;
    let (result, overflow) = r1v.overflowing_add(r2v);
    vm.set_partial_register_value(rr, result as u64);
    vm.ov = overflow;
}

pub fn u32_rir(vm: &mut Vm, r1: VmHwordRegister, i: u32, rr: VmHwordRegister) {
    let r1v = vm.get_partial_register_value(r1) as u32;
    let (result, overflow) = r1v.overflowing_add(i);
    vm.set_partial_register_value(rr, result as u64);
    vm.ov = overflow;
}

pub fn u32_rrr(vm: &mut Vm, r1: VmHwordRegister, r2: VmHwordRegister, rr: VmHwordRegister) {
    let r1v = vm.get_partial_register_value(r1) as u32;
    let r2v = vm.get_partial_register_value(r2) as u32;
    let (result, overflow) = r1v.overflowing_add(r2v);
    vm.set_partial_register_value(rr, result as u64);
    vm.ov = overflow;
}

pub fn f64_rir(vm: &mut Vm, r1: VmRegister, i: f64, rr: VmRegister) {
    let r1v = f64::from_bits(vm.get_register_value(r1));
    let result = r1v + i;
    vm.set_register_value(rr, result.to_bits());
    vm.ov = false;
}

pub fn f64_rrr(vm: &mut Vm, r1: VmRegister, r2: VmRegister, rr: VmRegister) {
    let r1v = f64::from_bits(vm.get_register_value(r1));
    let r2v = f64::from_bits(vm.get_register_value(r2));
    let result = r1v + r2v;
    vm.set_register_value(rr, result.to_bits());
    vm.ov = false;
}

pub fn i64_rir(vm: &mut Vm, r1: VmRegister, i: i64, rr: VmRegister) {
    let r1v = vm.get_register_value(r1) as i64;
    let (result, overflow) = r1v.overflowing_add(i);
    vm.set_register_value(rr, result as u64);
    vm.ov = overflow;
}

pub fn i64_rrr(vm: &mut Vm, r1: VmRegister, r2: VmRegister, rr: VmRegister) {
    let r1v = vm.get_register_value(r1) as i64;
    let r2v = vm.get_register_value(r2) as i64;
    let (result, overflow) = r1v.overflowing_add(r2v);
    vm.set_register_value(rr, result as u64);
    vm.ov = overflow;
}

pub fn u64_rir(vm: &mut Vm, r1: VmRegister, i: u64, rr: VmRegister) {
    let r1v = vm.get_register_value(r1);
    let (result, overflow) = r1v.overflowing_add(i);
    vm.set_register_value(rr, result);
    vm.ov = overflow;
}

pub fn u64_rrr(vm: &mut Vm, r1: VmRegister, r2: VmRegister, rr: VmRegister) {
    let r1v = vm.get_register_value(r1);
    let r2v = vm.get_register_value(r2);
    let (result, overflow) = r1v.overflowing_add(r2v);
    vm.set_register_value(rr, result);
    vm.ov = overflow;
}
