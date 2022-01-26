# `add.{numeric} $r1 #i`

# `add.{numeric} $r1 #i $rr`

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 0000000s r1...... i....... ........ ........ ........ rr......

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 0000000s r1...... 11111110 rr...... 00000000 00000000 00000000
     i....... ........ ........ ........ ........ ........ ........ ........

-   `s` indicates whether the operands are signed (1) or not (0)
-   `r1` The first operand register
-   `i` The inlined [immediate](../datatypes.md#immediates) value
-   `rr` The result storage register &mdash; Defaults to r1 if omitted

Adds together the values from `r1` and `i` and stores the result in `rr`.

# `add.{numeric} $r1 $r2`

# `add.{numeric} $r1 $r2 $rr`

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 0000001s r1...... r2...... rr...... 00000000 00000000 00000000

-   `s` indicates whether the operands are signed (1) or not (0)
-   `r1` The first operand register
-   `r2` The second operand register
-   `rr` The result storage register &mdash; Defaults to r1 if omitted

Adds together the values from `r1` and `r2` and stores the result in `rr`.
