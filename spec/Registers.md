### Registers

```
x   -> 111000    (6-bit full encoding), 1000 (4-bit word encoding)
xh0 -> 1010000   (7-bit full encoding)
xh1 -> 1010001
xq0 -> 01100000  (8-bit full encoding)
xq3 -> 01100011
xb0 -> 001000000 (9-bit full encoding)
xb7 -> 001000111
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
```

...which is nice because the physical register is always in the same spot! Oh, and the
index is just the remainder of dividing the highest 4 bits by the number of register
indexes.
