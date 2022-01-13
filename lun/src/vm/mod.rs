pub mod arithmetic;
pub mod err;
pub mod flow;
pub mod instruction;
pub mod io;
pub mod is;
pub mod logic;

pub use err::{VmError, VmResult};
pub use instruction::Instruction;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Default)]
pub struct Vm {
    p: u64, // program counter
    q: u64, // used for returns values
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

    eq: bool,
    ng: bool,
    ov: bool,

    halt: bool,
    result: VmResult,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u64)]
pub enum VmRegister {
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
}

impl Vm {
    pub fn get_register_value(&self, reg: VmRegister) -> u64 {
        use VmRegister::*;

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
        }
    }

    pub fn set_register_value(&mut self, reg: VmRegister, val: u64) {
        use VmRegister::*;

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
        };
    }

    pub fn update_register_value(&mut self, reg: VmRegister, reducer: fn(u64) -> u64) {
        let prev = self.get_register_value(reg);
        self.set_register_value(reg, reducer(prev));
    }

    pub fn exec(&mut self, instruction: impl Instruction) {
        instruction.exec(self);
    }

    pub fn inspect(&self) {
        println!("{:?}", self);
    }

    pub fn panic(&mut self, err: VmError) {
        self.halt = true;
        self.result = VmResult::Err(err);
        self.inspect();
    }
}
