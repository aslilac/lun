use super::{Vm, VmRegister};

impl Vm {
    pub fn xor_r(&mut self, r1: VmRegister) {
        self.set_register_value(r1, 0);
    }

    pub fn xor_rrr(&mut self, r1: VmRegister, r2: VmRegister, rr: VmRegister) {
        let r1v = self.get_register_value(r1);
        let r2v = self.get_register_value(r2);
        self.set_register_value(rr, r1v ^ r2v);
    }
}
