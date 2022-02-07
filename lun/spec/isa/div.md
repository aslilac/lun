# `div.{float} $r1 #i`

# `div.{float} $r1 #i $rr`

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 0000000s r1...... i....... ........ ........ ........ rr......

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 0000000s r1...... 11111110 rr...... 00000000 00000000 00000000
     i....... ........ ........ ........ ........ ........ ........ ........

-   `s` indicates whether the operands are signed (1) or not (0)
-   `r1` The first operand register
-   `i` The inlined [immediate](../datatypes.md#immediates) value
-   `rr` The register to store the result in &mdash; Defaults to r1 if omitted

Divides the value from `r1` by the value from `i`, stores the result in `rr`.

# `div.{float} $r1 $r2`

# `div.{float} $r1 $r2 $rr`

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 0000000s r1...... r2...... rr...... 00000000 00000000 00000000

-   `s` indicates whether the operands are signed (1) or not (0)
-   `r1` The first operand register
-   `r2` The second operand register
-   `rr` The register to store the result in &mdash; Defaults to r1 if omitted

Divides the value from `r1` by the value from `r2`, stores the result in `rr`.

# `div.{integer} $r1 #i $rr -`

# `div.{integer} $r1 #i - $rm`

# `div.{integer} $r1 #i $rr $rm`

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 000000ms r1...... i....... ........ ........ ........ rr......

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 0000000s r1...... 11111110 rr...... rm...... 00000000 00000000
     i....... ........ ........ ........ ........ ........ ........ ........

Divides the value from `r1` by the value from `i`, stores the quotient in `rr` and the
remainder (otherwise called the modulus) in `rm`.

# `div.{integer} $r1 $r2`

# `div.{integer} $r1 $r2 $rr -`

# `div.{integer} $r1 $r2 - $rm`

# `div.{integer} $r1 $r2 $rr $rm`

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 0000001s r1...... r2...... rr...... rm...... 00000000 00000000

-   `s` indicates whether the operands are signed (1) or not (0)
-   `r1` The first operand register
-   `r2` The second operand register
-   `rr` The result storage register for the quotient &mdash; Defaults to r1 if omitted
-   `rm` The result storage register for the remainder &mdash; Defaults to r2 if omitted

Divides the value from `r1` by the value from `r2`, stores the quotient in `rr` and the
remainder (otherwise called the modulus) in `rm`.
