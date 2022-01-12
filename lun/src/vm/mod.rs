pub mod arithmetic;
pub mod instruction;
pub mod logic;

pub use instruction::Instruction;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Default)]
pub struct Vm {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
    e: u64,
    f: u64,
    x: u64,
    y: u64,

    eq: bool,
    ng: bool,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u64)]
pub enum VmRegister {
    x = 0b1000,
    y = 0b1001,
    a = 0xa,
    b = 0xb,
    c = 0xc,
    d = 0xd,
    e = 0xe,
    f = 0xf,
}

impl Vm {
    pub fn get_register_value(&self, reg: VmRegister) -> u64 {
        use VmRegister::*;

        match reg {
            a => self.a,
            b => self.b,
            c => self.c,
            d => self.d,
            e => self.e,
            f => self.f,
            x => self.x,
            y => self.y,
        }
    }

    pub fn set_register_value(&mut self, reg: VmRegister, val: u64) {
        use VmRegister::*;

        match reg {
            a => self.a = val,
            b => self.b = val,
            c => self.c = val,
            d => self.d = val,
            e => self.e = val,
            f => self.f = val,
            x => self.x = val,
            y => self.y = val,
        };
    }

    pub fn exec(&mut self, instruction: impl Instruction) {
        instruction.exec(self);
    }

    pub fn inspect(&self) {
        println!("{:?}", self);
    }
}
