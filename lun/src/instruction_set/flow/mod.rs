mod br;
mod halt;

use super::iprelude::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FlowInstruction {
    br_i(i64),
    halt_x_x,
}

impl Instruction for FlowInstruction {
    fn exec(self, vm: &mut Vm) {
        use FlowInstruction::*;

        match self {
            br_i(i) => br::i(vm, i),
            halt_x_x => halt::x_x(vm),
        }
    }
}
