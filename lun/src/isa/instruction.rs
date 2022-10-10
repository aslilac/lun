use crate::vm::Vm;

pub trait Instruction {
	fn exec(self, vm: &mut Vm);
}
