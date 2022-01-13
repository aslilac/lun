mod br;

use crate::vm::{Instruction, Vm, VmRegister};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FlowInstruction {
    br_i(i64),
}

impl Instruction for FlowInstruction {
    fn exec(self, vm: &mut Vm) {
        use FlowInstruction::*;

        match self {
            br_i(i) => br::i(vm, i),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FlowInstruction::*, Vm, VmRegister::*};
    
    #[test]
    fn br() {
        let mut vm = Vm::default();
    }
}
