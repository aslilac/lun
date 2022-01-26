### Registers

```
x   -> 1000       (4-bit word encoding)
x   -> 1110_1000  (8-bit full encoding)
xh0 -> 1100_1000
xh1 -> 1101_1000
xq0 -> 1001_1000
xq3 -> 1011_1000
xb0 -> 0000_1000
xb7 -> 0111_1000
```

```
16 registers                       16
16 registers * 2 half indexes    + 32  = 48
16 registers * 4 quarter indexes + 64  = 112
16 registers * 8 byte indexes    = 128 = 240
```

Could theoretically be coded as 8 bits... is saving the 1 bit worth it? 🤔

```
00000000 .. 01111111 -> pb0 .. fb7 (0rrrriii)
10000000 .. 10111111 -> pq0 .. fq3 (10rrrrii)
11000000 .. 11011111 -> ph0 .. fh1 (110rrrri)
11100000 .. 11101111 -> p   .. f   (1110rrrr)
```

...or a slightly easier to parse version...

```
00000000 .. 01111111 -> pb0 .. fb7 (0iiirrrr)
10000000 .. 10111111 -> pq0 .. fq3 (10iirrrr)
11000000 .. 11011111 -> ph0 .. fh1 (110irrrr)
11100000 .. 11101111 -> p   .. f   (1110rrrr)
11110000 .. 11111110 -> undefined behavior
11111100 .. 11111111 -> fwi        (full word immediate)
11111111             -> None       (rarely useful, but worth having)
```

...which is nice because the physical register is always in the same spot! Oh, and the
index is just the remainder of dividing the highest 4 bits by the number of register
indexes.
