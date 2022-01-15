#![feature(mixed_integer_ops)]
// Remove ASAP
#![allow(dead_code)]

mod instruction_set;
mod prelude;
mod vm;

fn main() -> Result<(), std::io::Error> {
    use prelude::*;

    let mut vm = Vm::default();

    // vm.set_register_value(x, 0x0706050403020100);
    // println!(
    //     "xb0|xu8_0: {}",
    //     vm.get_partial_register_value(VmByteRegister::new(x, 0))
    // );
    // println!(
    //     "xb1|xu8_1: {}",
    //     vm.get_partial_register_value(VmByteRegister::new(x, 1))
    // );
    // println!(
    //     "xb2|xu8_2: {}",
    //     vm.get_partial_register_value(VmByteRegister::new(x, 2))
    // );
    // println!(
    //     "xb3|xu8_3: {}",
    //     vm.get_partial_register_value(VmByteRegister::new(x, 3))
    // );
    // println!(
    //     "xb4|xu8_4: {}",
    //     vm.get_partial_register_value(VmByteRegister::new(x, 4))
    // );
    // println!(
    //     "xb5|xu8_5: {}",
    //     vm.get_partial_register_value(VmByteRegister::new(x, 5))
    // );
    // println!(
    //     "xb6|xu8_6: {}",
    //     vm.get_partial_register_value(VmByteRegister::new(x, 6))
    // );
    // println!(
    //     "xb7|xu8_7: {}",
    //     vm.get_partial_register_value(VmByteRegister::new(x, 7))
    // );

    vm.set_register_value(a, 0x6173206f6c6c6548); // Hello sa
    vm.exec(put_r(a));
    vm.set_register_value(a, 0x0a21726f6c69); // ilor!\n
    vm.exec(put_r(a));

    // vm.inspect();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn random_arithmatic_test() {
        let mut vm = Vm::default();

        // Initialize to 0, use as a counter
        vm.exec(xor_r(a));
        assert_eq!(0, vm.get_register_value(a));

        // Initialize to 0, use as an accumulator
        vm.exec(xor_r(b));

        // Initialize to 0, used to find fibbonacci numbers
        vm.exec(xor_r(c));
        vm.exec(xor_r(d));
        vm.exec(xor_r(e));

        vm.exec(add_u64_ri(c, 1)); // 0 + 1 = 1
        vm.exec(add_u64_ri(d, 1)); // 0 + 1 = 1

        for _ in 0..10 {
            // Increment a
            vm.exec(add_u64_ri(a, 1));

            // b accumulates a
            vm.exec(add_u64_rr(b, a));

            // [c, d, e] = [d, e, c + d]
            vm.exec(add_u64_rrr(c, d, e));
            vm.exec(add_u64_rir(d, 0, c));
            vm.exec(add_u64_rir(e, 0, d));

            println!(
                "[ a: {: <5}, b: {: <5}, c: {: <5}, d: {: <5}, e: {: <5} ]",
                vm.get_register_value(a),
                vm.get_register_value(b),
                vm.get_register_value(c),
                vm.get_register_value(d),
                vm.get_register_value(e),
            );
        }

        assert_eq!(10, vm.get_register_value(a));
        assert_eq!(55, vm.get_register_value(b));
        assert_eq!(89, vm.get_register_value(c));
        assert_eq!(144, vm.get_register_value(d));
        assert_eq!(144, vm.get_register_value(e));

        // Clear to 0 (short form)
        vm.exec(xor_r(a));
        assert_eq!(0, vm.get_register_value(a));

        // Make it not 0 again
        vm.exec(add_u64_ri(a, 10));
        assert_eq!(10, vm.get_register_value(a));

        // Clear to 0 (long form)
        vm.exec(xor_rrr(a, a, a));
        assert_eq!(0, vm.get_register_value(a));
    }
}
