# Overview

# Words

Wherever possible we try to stick to keeping everything as a 64 bit value. Throughout
the specifications of the VM, a word should be interpreted to mean a 64 bit value.
Similarly a half word (or hword) should be interpreted to mean a 32 bit value.

# Instructions

See Is.md

# Registers

Each core has a bank of 8 general purpose registers, as well as a couple of internal
registers. The general purpose registers are called a, b, c, d, e, f, x, and y.
The internal registers are call l, u, and n.

-   l: exec pointer
-   u: flags and stuff
-   n: stack pointer

# Privledge

The VM has a few different priviledge rankings that can be used to isolate processes.

# Memory

Memory is word addressable, and uses a word as its address space. This gives a theoretical
maximum memory space of 2^64 words of memory. This gives us a theoretical maximum of 32
exabytes of data, which is large.

    2^10 * 2^10 * 2^10 * 2^10 * 2^10 * 2^10 * 2^4
    ^ K    ^ M    ^ G    ^ T    ^ P    ^ E    ^ 8    ^ 8 (64 bits -> 8 bytes)
