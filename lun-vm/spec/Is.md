# General design

Many instructions have variants (denoted by a suffix of a `.` and the variant name). As
an example, the typical signed integer addition instruction is written as `add.i64`. The
instruction is `add` and the variant is `i64`, meaning it is intended to operate on signed
64 bit values.

# Syntax

The descriptions given make use of a variant of Lunar Assembly (LA) to show the options of
how each instruction might be written in assembly by an engineer (and in turn assembled to
Lunar Machine-code (LM)).

`{a,b,c,...}` means one of any of the comma separated values

    add.{i64,u64} -> add.i64 | add.u64

`:_` means one of any of the general purpose registers (a, b, c, d, e, f, x, y)

`:!` means one of any of the internal registers (l, u, n)

`:a` means the a register, `:b` means the b register, etc.
`:l` means the l internal register, `:u` means the u internal register, etc.

`#` means any number (in any of the supported forms)

-   Decimal numbers can be used unprefixed
-   Hex numbers should use the `0x` prefix
-   Binary numbers should use the `0b` prefix

# Binary representation

# Opcode

LM is designed to have a somewhat simple instruction set, and most importantly is designed
to be easily (relatively) understood by a human, since it's mostly just for fun. As such,
the opcode refers to the most significant two bytes of the first word of an instruction.
Variants should be determined by the next four most significant bits (or in some cases,
the two most significant bits of the opcode, which determine instruction size)

## Registers

A register is always represented by 4 bits.

    0000 l
    0001 u
    0010 n
    ------
    1000 x
    1001 y
    1010 a
    1011 b
    1100 c
    1101 d
    1110 e
    1111 f

It is worthwhile to note 2 things

-   The binary representation of registers a-f corresponds with their representation in hexadecimal.
    i.e. `a` in hexidecimal is `1010` in binary, which is the binary value that represents the a register.
-   All of the general purpose registers are represented with a most-significant bit of 1.
    This also means that all of the internal registers are represented with a most-significant
    bit of 0.

## Instruction size

Instructions are all stored as words. Some instructions can be stored across
multiple words, up to 5. The number of words an instruction is encoded as part of the
opcode, which is always contained in the first word.

`00` - 1 word
`01` - 2 words
`10` - 4 words
`11` - 5 words

# Arithmatic

`add.{i64,u64} :_ #`

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 00000000 100s0000 0000r1.. nnnnnnnn nnnnnnnn nnnnnnnn nnnnnnnn

    : s	    0 => u64, 1 => i64
    : r1..	The first operand register, and the result storage register
    : n	    The inlined immediate value (a 32 bit value that will be signed extended to 64 bits)

Adds together the numbers from the first register and the inlined immediate value and
stores them in the first register.

`add.{i64,u64} :_ :_`

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 00000000 110s0000 0000r1.. 0000r2.. 00000000 00000000 00000000

    : s	    0 => u64, 1 => i64
    : r1..	The first operand register, and the result storage register
    : r2..	The second operand register

Adds together the numbers from the first and second given registers and stores them in the
first register.

`add.{i64,u64} :_ :_ :_`

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 00000000 111s0000 0000r1.. 0000r2.. 0000r3.. 00000000 00000000

    : s	    0 => u64, 1 => i64
    : r1..  The first operand register
    : r2..  The second operand register
    : rr..  The result storage register

Adds together the numbers from the first and second given registers and stores them in the
third register.

`xor :_`

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 00000001 10010000 0000r... 00000000 00000000 00000000 00000000

    : r...  The register to clear

Shorthand for `xor :_ :_ :_` (useful for zeroing a register)

`xor :_ :_ :_`

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00000000 00000001 11100000 0000r1.. 0000r2.. 0000r3.. 00000000 00000000

    : r1..  The first operand register
    : r2..  The second operand register
    : rr..  The result storage register

Performs an exclusive or of the numbers from the first and second given registers and
stores them in the third register.

# Memory

`i.{at,by} :_ #`
`i.at :_ :_`
`i.by :_ [label]`

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00100000 00000000 10000000 0000ri.. dddddddd dddddddd dddddddd dddddddd

    : ri.. The register to inload from memory
    : d    The offset from the address of the word containing this instruction

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00100000 00000000 11000000 0000ri.. 0000r@.. 00000000 00000000 00000000

    : ri.. The register to inload from memory
    : r@.. The register containing the address in memory to read from

    |--------|--------|--------|--------|--------|--------|--------|--------|
     01100000 00000000 10000000 0000ri.. 00000000 00000000 00000000 00000000
     @@@@@@@@ @@@@@@@@ @@@@@@@@ @@@@@@@@ @@@@@@@@ @@@@@@@@ @@@@@@@@ @@@@@@@@

    : ri.. The register to inload from memory
    : @    The address in memory to read from

Reads a value from an address in memory into the specified register

`o.{at,by} :_ #`
`o.at :_ :_`
`o.by :_ [label]`

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00010000 00000000 10000000 0000ro.. dddddddd dddddddd dddddddd dddddddd

    : ro.. The register to outload to memory
    : d    The offset from the address of the word containing this instruction

    |--------|--------|--------|--------|--------|--------|--------|--------|
     00100000 00000000 11000000 0000ro.. 0000r@.. 00000000 00000000 00000000

    : ro.. The register to outload to memory
    : r@.. The register containing the address in memory to write to

    |--------|--------|--------|--------|--------|--------|--------|--------|
     01010000 00000000 10000000 0000ro.. 00000000 00000000 00000000 00000000
     @@@@@@@@ @@@@@@@@ @@@@@@@@ @@@@@@@@ @@@@@@@@ @@@@@@@@ @@@@@@@@ @@@@@@@@

    : ro.. The register to outload to memory
    : @    The address in memory to write to

Writes a value from the specified register into memory at an address
