# `add.{numeric} $r1 #i`

# `add.{numeric} $r1 #i $rr`

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 0000000s rr...... r1...... i....... ........ ........ ........

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 0000000s rr...... 11110000 r1...... 00000000 00000000 00000000
     i....... ........ ........ ........ ........ ........ ........ ........

- `s` indicates whether the operands are signed (`1`) or not (`0`)
- `r1` The first operand register
- `i` The inlined [immediate](../datatypes.md#immediates) value
- `rr` The register to store the result in &mdash; Defaults to r1 if omitted

Adds together the values from `r1` and `i`, stores the result in `rr`.

# `add.{numeric} $r1 $r2`

# `add.{numeric} $r1 $r2 $rr`

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 0000001s r1...... r2...... rr...... 00000000 00000000 00000000

- `s` indicates whether the operands are signed (`1`) or not (`0`)
- `r1` The first operand register
- `r2` The second operand register
- `rr` The register to store the result in &mdash; Defaults to r1 if omitted

Adds together the values from `r1` and `r2`, stores the result in `rr`.
