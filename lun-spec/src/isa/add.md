# `add.{i$size,u$size} $r1 #i`

# `add.{i$size,u$size} $r1 #i $rr`

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 0000000s r1...... iiiiiiii iiiiiiii iiiiiiii iissssss rr......

# `add.{i$size,u$size} $r1 $r2`

# `add.{i$size,u$size} $r1 $r2 $rr`

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 0000001s r1...... r2...... rr......{00000000 00000000 00000000}

-   `s` 0 => u$size, 1 => i$size
-   `r1` The first operand register
-   i The inlined immediate value
    -   When $size is 64, this is a 26-bit value, with a 6-bit rotation factor
    -   When $size is 32 or less, it is a fully represented inlined value
    -   When $size is less than 32, the value should be stored in the least
        significant bits, and unneeded precision should be zeroed.
        For example, an i8 of -1 should be `00000000 00000000 00000000 11111111`.
-   `r2` The second operand register
-   rr.. The result storage register &mdash; Defaults to r1 if omitted

Adds together the values from r1 and the inlined immediate value and stores the
result in rr. rr can be comma

-   `s` indicates whether the operands are signed or not
-   `r1` The first operand register
-   `rr` The result storage register

Adds together the numbers from the first and second given registers and stores them in the
third register.
