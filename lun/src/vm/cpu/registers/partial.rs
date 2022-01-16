use super::VmRegister;
use std::{fmt, fmt::Debug};

pub trait PartialRegister {
    // TODO: Is there some way to do this using intrinsics?
    const MAX_REGISTER_INDEX: u64;
    const MASK: u64;
    const WIDTH: u64;

    fn get_reg_and_index(self) -> (VmRegister, u64);
}

#[derive(Copy, Clone, PartialEq)]
pub struct VmByteRegister(VmRegister, u64);

impl VmByteRegister {
    pub fn new(reg: VmRegister, i: u64) -> Self {
        if cfg!(debug_assertions) {
            assert!(i < Self::MAX_REGISTER_INDEX);
        }

        Self(reg, i)
    }
}

impl PartialRegister for VmByteRegister {
    const MAX_REGISTER_INDEX: u64 = 8;
    const MASK: u64 = 0xff;
    const WIDTH: u64 = 8;

    fn get_reg_and_index(self) -> (VmRegister, u64) {
        (self.0, self.1)
    }
}

impl Debug for VmByteRegister {
    fn fmt(&self, disp: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(disp, "{:?}b{}", self.0, self.1)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VmQwordRegister(VmRegister, u64);

impl VmQwordRegister {
    const MAX_REGISTER_INDEX: u64 = 4;

    pub fn new(reg: VmRegister, i: u64) -> Self {
        if cfg!(debug_assertions) {
            assert!(i < Self::MAX_REGISTER_INDEX);
        }

        Self(reg, i)
    }
}

impl PartialRegister for VmQwordRegister {
    const MAX_REGISTER_INDEX: u64 = 4;
    const MASK: u64 = 0xffff;
    const WIDTH: u64 = 16;

    fn get_reg_and_index(self) -> (VmRegister, u64) {
        (self.0, self.1)
    }
}

impl Debug for VmQwordRegister {
    fn fmt(&self, disp: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(disp, "{:?}q{}", self.0, self.1)
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VmHwordRegister(VmRegister, u64);

impl VmHwordRegister {
    const MAX_REGISTER_INDEX: u64 = 2;

    pub fn new(reg: VmRegister, i: u64) -> Self {
        if cfg!(debug_assertions) {
            assert!(i < Self::MAX_REGISTER_INDEX);
        }

        Self(reg, i)
    }
}

impl PartialRegister for VmHwordRegister {
    const MAX_REGISTER_INDEX: u64 = 2;
    const MASK: u64 = 0xffffffff;
    const WIDTH: u64 = 32;

    fn get_reg_and_index(self) -> (VmRegister, u64) {
        (self.0, self.1)
    }
}

impl Debug for VmHwordRegister {
    fn fmt(&self, disp: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(disp, "{:?}h{}", self.0, self.1)
    }
}

impl PartialRegister for VmRegister {
    const MAX_REGISTER_INDEX: u64 = 1;
    const MASK: u64 = u64::MAX;
    const WIDTH: u64 = 64;

    fn get_reg_and_index(self) -> (VmRegister, u64) {
        (self, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{Vm, VmRegister::*};

    #[test]
    fn byte_registers() {
        let mut vm = Vm::default();
        vm.set_register_value(x, 0x0706050403020100);

        for i in 0..8 {
            assert_eq!(vm.get_partial_register_value(VmByteRegister::new(x, i)), i);
            vm.set_partial_register_value(VmByteRegister::new(y, i), i);
        }

        assert_eq!(vm.get_register_value(y), 0x0706050403020100);

        vm.set_partial_register_value(VmByteRegister::new(y, 4), 0xff);
        assert_eq!(vm.get_register_value(y), 0x070605ff03020100);
    }
}
