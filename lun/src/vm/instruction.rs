use super::Vm;
pub trait Instruction {
    fn exec(self, vm: &mut Vm);
}
