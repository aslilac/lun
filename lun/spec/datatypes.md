### Integers

-   i8
-   u8
-   i16
-   u16
-   i32
-   u32
-   i64
-   u64

Integers are encoded in the usual way. Unsigned integers count up starting from zero,
and signed intergers are stored in two's compliment fashion.

### Floats

-   f32
-   f64

Floating point numbers are stored using the typical IEEE 754 standard.

### Words

-   byte (8 bits)
-   qword (16 bits)
-   hword (32 bits)
-   word (64 bits)

Words are just generic bits of data, without any preassigned meaning. They can be whatever
you want! Might be text, might represent a color, or something more!

### Immediates

#### Word immediate

    |--------|--------|--------|--------|--------|--------|--------|--------|
     iiiiiiii iiiiiiii iiiiiiii iiiiiiii iiiiiiii iiiiiiii iiiiiiii iiiiiiii

#### Hword immediate

    |--------|--------|--------|--------|
     iiiiiiii iiiiiiii iiiiiiii iirrrrrr

    |--------|--------|--------|--------|
     iiiiiiii iiiiiiii iiiiiiii iiiiiiii

    |--------|--------|--------|--------|
     00000000 00000000 iiiiiiii iiiiiiii

    |--------|--------|--------|--------|
     00000000 00000000 00000000 iiiiiiii

-   `i` is the binary content of the immediate value
-   `s` is an unsigned 6-bit value that indicates how much `i` should be left-rotated to
    form the true value of the encoded immediate (only applicable to hword immediates
    which are encoding a word value.

The general logic behind this encoding is...

-   If the size of the word being stored is less than or equal to the available size for
    encoding the immediate, it should be stored exactly, aligned to the least significant
    digits, and padded with zeroes in the unused most significant digits, including cases
    where the value is a signed integer.

-   If the size of the word being stored is greater than the available size for encoding
    the immediate, then the first n least significant bits should be used to store an
    unsigned integer indicating how many digits to the left the partial value should be
    rotated, where s is the size of the word in bits and n is ⌈log2(s)⌉, and the remaining
    available bits should be used to store exact binary data.

A few examples...

-   An i8 with a value of -1 would be encoded as `00000000 00000000 00000000 11111111`
-   An i64,u64 with a value of 1 could be encoded as `00000000 00000000 00000000 01000000`
-   An i64,u64 with a value of 0xf0 could be encoded as `00000000 00000000 00000011 11000100`
-   A u64 with a value of 0x8000000000000000 could be encoded as `00000000 00000000 00000000 01111111`

### Caveats

-   f64 values cannot be stored in an hword immediate, and must be addressed as a [fwi](#).
