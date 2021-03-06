mod addressable;

pub use addressable::*;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct RegisterSet {
    p: u64, // program counter
    q: u64, // used for return values
    r: u64, // used for return values
    s: u64, // stack pointer
    t: u64, // use for timers?
    u: u64,
    v: u64,
    w: u64,

    a: u64,
    b: u64,
    c: u64,
    d: u64,
    e: u64,
    f: u64,
    x: u64,
    y: u64,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u64)]
pub enum VmNativeRegister {
    p = 0b0000,
    q = 0b0001,
    r = 0b0010,
    s = 0b0011,
    t = 0b0100,
    u = 0b0101,
    v = 0b0110,
    w = 0b0111,
    x = 0b1000,
    y = 0b1001,
    a = 0xa,
    b = 0xb,
    c = 0xc,
    d = 0xd,
    e = 0xe,
    f = 0xf,
    sink,
}

pub trait VmRegister = AddressableRegister + Copy + Debug + Sized;

impl RegisterSet {
    pub fn get_native_register_value(&self, reg: VmNativeRegister) -> u64 {
        use VmNativeRegister::*;

        match reg {
            p => self.p,
            q => self.q,
            r => self.r,
            s => self.s,
            t => self.t,
            u => self.u,
            v => self.v,
            w => self.w,
            x => self.x,
            y => self.y,
            a => self.a,
            b => self.b,
            c => self.c,
            d => self.d,
            e => self.e,
            f => self.f,
            sink => unreachable!("VmRegister::sink must only be used as a destination"),
        }
    }

    pub fn get_register_value<T: VmRegister>(&self, reg: T) -> u64 {
        let (native_reg, i) = reg.get_reg_and_index();
        let native_reg_value = self.get_native_register_value(native_reg);

        if cfg!(debug_assertions) {
            assert!(i < T::MAX_REGISTER_INDEX);
        }

        // TODO: Make this work
        // match reg {
        //     VmNativeRegister => native_reg_value,
        //     _ => (native_reg_value >> (i * T::WIDTH)) & T::MASK,
        // }
        (native_reg_value >> (i * T::WIDTH)) & T::MASK
    }

    pub fn set_native_register_value(&mut self, reg: VmNativeRegister, val: u64) {
        use VmNativeRegister::*;

        match reg {
            p => self.p = val,
            q => self.q = val,
            r => self.r = val,
            s => self.s = val,
            t => self.t = val,
            u => self.u = val,
            v => self.v = val,
            w => self.w = val,
            x => self.x = val,
            y => self.y = val,
            a => self.a = val,
            b => self.b = val,
            c => self.c = val,
            d => self.d = val,
            e => self.e = val,
            f => self.f = val,
            sink => (),
        };
    }

    pub fn set_register_value<T: VmRegister>(&mut self, reg: T, val: u64) {
        // TODO: If reg is a native reg we should be able to just set immediately
        let (native_reg, i) = reg.get_reg_and_index();
        let native_reg_value = self.get_native_register_value(native_reg);

        if cfg!(debug_assertions) {
            assert!(i < T::MAX_REGISTER_INDEX);
        }

        let cleared_reg_value = native_reg_value & !(T::MASK << (i * T::WIDTH));

        let sanitized_partial_word = (val & T::MASK) << (i * T::WIDTH);
        let set_value = cleared_reg_value | sanitized_partial_word;
        self.set_native_register_value(native_reg, set_value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::VmNativeRegister::*;

    #[test]
    fn register_names() {
        assert_eq!(format!("{:?}", VmByteRegister::new(x, 0)), "xb0");
        assert_eq!(format!("{:?}", VmQwordRegister::new(x, 0)), "xq0");
        assert_eq!(format!("{:?}", VmHwordRegister::new(x, 0)), "xh0");
        assert_eq!(format!("{:?}", x), "x");
    }
}
