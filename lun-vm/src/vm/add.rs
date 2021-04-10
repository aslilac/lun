use super::{Vm, VmRegister};

impl Vm {
    pub fn add_u64_ri(&mut self, r1: VmRegister, i: u64) {
        let r1v = self.get_register_value(r1);
        self.set_register_value(r1, r1v.overflowing_add(i).0);
    }

    pub fn add_u64_rir(&mut self, r1: VmRegister, i: u64, rr: VmRegister) {
        let r1v = self.get_register_value(r1);
        self.set_register_value(rr, r1v.overflowing_add(i).0);
    }

    pub fn add_u64_rr(&mut self, r1: VmRegister, r2: VmRegister) {
        let r1v = self.get_register_value(r1);
        let r2v = self.get_register_value(r2);
        self.set_register_value(r1, r1v.overflowing_add(r2v).0);
    }

    pub fn add_u64_rrr(&mut self, r1: VmRegister, r2: VmRegister, rr: VmRegister) {
        let r1v = self.get_register_value(r1);
        let r2v = self.get_register_value(r2);
        self.set_register_value(rr, r1v.overflowing_add(r2v).0);
    }
}
