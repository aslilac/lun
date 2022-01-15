pub mod cpu;
pub mod err;
pub mod prelude;

use crate::base::{DisplayBuffer, DisplayTarget};
use crate::isa::prelude::Instruction;
pub use cpu::registers::{
    PartialRegister, RegisterSet, VmByteRegister, VmHwordRegister, VmQwordRegister, VmRegister,
    VmRegister::*,
};
pub use err::{VmError, VmResult};
use std::fmt::Debug;

// TODO: Remove the pub(crate) bits from this, replace with pub methods
#[derive(Debug)]
pub struct Vm {
    rs: RegisterSet,

    pub(crate) mem: Vec<u64>,
    pub(crate) disp: DisplayBuffer,

    pub(crate) eq: bool,
    pub(crate) ng: bool,
    pub(crate) ov: bool,

    pub(crate) halt: bool,
    pub(crate) result: VmResult,
}

impl Default for Vm {
    fn default() -> Self {
        Self {
            rs: Default::default(),

            mem: Default::default(),
            disp: DisplayTarget::buffer_with_capacity(Self::DISPLAY_BUFFER_CAPACITY),

            eq: Default::default(),
            ng: Default::default(),
            ov: Default::default(),
            halt: Default::default(),
            result: Default::default(),
        }
    }
}

impl Vm {
    const DISPLAY_BUFFER_CAPACITY: usize = 1024;
    const DEFAULT_MEMORY_CAPACITY: usize = 1024;

    pub fn new() -> Self {
        Self {
            rs: Default::default(),

            mem: Vec::with_capacity(Self::DEFAULT_MEMORY_CAPACITY),
            disp: DisplayTarget::buffer_with_capacity(Self::DISPLAY_BUFFER_CAPACITY),

            eq: Default::default(),
            ng: Default::default(),
            ov: Default::default(),
            halt: Default::default(),
            result: Default::default(),
        }
    }

    pub fn get_register_value(&self, reg: VmRegister) -> u64 {
        self.rs.get_register_value(reg)
    }

    pub fn get_partial_register_value(&self, reg: impl PartialRegister) -> u64 {
        self.rs.get_partial_register_value(reg)
    }

    pub fn set_register_value(&mut self, reg: VmRegister, val: u64) {
        self.rs.set_register_value(reg, val);
    }

    pub fn set_partial_register_value(&mut self, reg: impl PartialRegister, val: u64) {
        self.rs.set_partial_register_value(reg, val)
    }

    pub fn update_register_value(
        &mut self,
        reg: VmRegister,
        reducer: impl FnOnce(u64) -> Result<u64, VmError>,
    ) -> Result<u64, VmError> {
        let prev = self.get_register_value(reg);
        let result = reducer(prev);

        match result {
            Ok(value) => self.set_register_value(reg, value),
            Err(error) => self.panic(error),
        };

        result
    }

    pub fn update_partial_register_value(
        &mut self,
        reg: impl PartialRegister + Copy,
        reducer: impl FnOnce(u64) -> Result<u64, VmError>,
    ) -> Result<u64, VmError> {
        let prev = self.get_partial_register_value(reg);
        let result = reducer(prev);

        match result {
            Ok(value) => self.set_partial_register_value(reg, value),
            Err(error) => self.panic(error),
        };

        result
    }

    pub fn clear_registers(&mut self) {
        self.rs = RegisterSet::default();
    }

    pub fn exec(&mut self, inst: impl Instruction + Debug) {
        println!("{:?}", inst);
        inst.exec(self);
    }

    pub fn inspect(&self) {
        println!("{:#x?}", self);
    }

    pub fn panic(&mut self, err: VmError) {
        self.halt = true;
        self.result = VmResult::Err(err);
        self.inspect();
    }
}
