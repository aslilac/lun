use crate::vm::{VmError::BrOverflow, Vm, VmRegister::p};

pub fn i(vm: &mut Vm, i: i64) {
    vm.update_register_value(p, &move |prev| {
        let result = prev.checked_add_signed(i);

        if let Some(value) = result {
            value
        } else {
            println!("encountered overflow while branching:\n {} + {} {}", prev, i, "resulted in unacceptable program counter overflow");
            vm.panic(BrOverflow);
            prev
        }
    });
}
