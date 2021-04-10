mod vm;

use vm::{Vm, VmRegister};

fn main() -> Result<(), std::io::Error> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::vm::{Vm, VmRegister::*};

    #[test]
    fn random_arithmatic_test() {
        let mut lun = Vm::default();

        // Initialize to 0, use as a counter
        lun.xor_r(a);
        assert_eq!(0, lun.get_register_value(a));

        // Initialize to 0, use as an accumulator
        lun.xor_r(b);

        // Initialize to 0, used to find fibbonacci numbers
        lun.xor_r(c);
        lun.xor_r(d);
        lun.xor_r(e);
        lun.add_u64_ri(c, 1); // 0 + 1 = 1
        lun.add_u64_ri(d, 1); // 0 + 1 = 1

        for _ in 0..10 {
            // Increment a
            lun.add_u64_ri(a, 1);

            // b accumulates a
            lun.add_u64_rr(b, a);

            // [c, d, e] = [d, e, c + d]
            lun.add_u64_rrr(c, d, e);
            lun.add_u64_rir(d, 0, c);
            lun.add_u64_rir(e, 0, d);

            // println!(
            //     "[ a: {: <5}, b: {: <5}, c: {: <5}, d: {: <5}, e: {: <5} ]",
            //     lun.get_register_value(a),
            //     lun.get_register_value(b),
            //     lun.get_register_value(c),
            //     lun.get_register_value(d),
            //     lun.get_register_value(e),
            // );
        }

        assert_eq!(10, lun.get_register_value(a));
        assert_eq!(55, lun.get_register_value(b));
        assert_eq!(89, lun.get_register_value(c));
        assert_eq!(144, lun.get_register_value(d));
        assert_eq!(144, lun.get_register_value(e));

        // Clear to 0 (short form)
        lun.xor_r(a);
        assert_eq!(0, lun.get_register_value(a),);

        // Make it not 0 again
        lun.add_u64_ri(a, 10);
        assert_eq!(10, lun.get_register_value(a),);

        // Clear to 0 (long form)
        lun.xor_rrr(a, a, a);
        assert_eq!(0, lun.get_register_value(a),);
    }
}
