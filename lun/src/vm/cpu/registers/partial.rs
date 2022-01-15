use super::VmRegister;

pub trait PartialRegister {
    // TODO: Is there some way to do this using intrinsics?
    const MAX_REGISTER_INDEX: u64;
    const MASK: u64;
    const WIDTH: u64;

    fn get_reg_and_index(self) -> (VmRegister, u64);
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::{Vm, VmRegister::*};

    fn byte_registers() {
        let mut vm = Vm::default();

        vm.set_register_value(x, 0x0706050403020100);

        assert_eq!(vm.get_partial_register_value(VmByteRegister::new(x, 0)), 0);
        assert_eq!(vm.get_partial_register_value(VmByteRegister::new(x, 1)), 1);
        assert_eq!(vm.get_partial_register_value(VmByteRegister::new(x, 2)), 2);
        assert_eq!(vm.get_partial_register_value(VmByteRegister::new(x, 3)), 3);
        assert_eq!(vm.get_partial_register_value(VmByteRegister::new(x, 4)), 4);
        assert_eq!(vm.get_partial_register_value(VmByteRegister::new(x, 5)), 5);
        assert_eq!(vm.get_partial_register_value(VmByteRegister::new(x, 6)), 6);
        assert_eq!(vm.get_partial_register_value(VmByteRegister::new(x, 7)), 7);

        vm.set_partial_register_value(VmByteRegister::new(y, 0), 0);
        vm.set_partial_register_value(VmByteRegister::new(y, 1), 1);
        vm.set_partial_register_value(VmByteRegister::new(y, 2), 2);
        vm.set_partial_register_value(VmByteRegister::new(y, 3), 3);
        vm.set_partial_register_value(VmByteRegister::new(y, 4), 4);
        vm.set_partial_register_value(VmByteRegister::new(y, 5), 5);
        vm.set_partial_register_value(VmByteRegister::new(y, 6), 6);
        vm.set_partial_register_value(VmByteRegister::new(y, 7), 7);

        assert_eq!(vm.get_register_value(y), 0x0706050403020100);

        vm.set_partial_register_value(VmByteRegister::new(y, 4), 0xff);

        assert_eq!(vm.get_register_value(y), 0x070605ff03020100);
    }
}
