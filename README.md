<!-- # <img alt="Lun" src="media/lun.png" height="200" /> -->
<h1 align="center"><img alt="Lun" src="media/lun.png" height="200" /></h1>

Lun is a virtual machine with its own instruction set, that's mostly just for fun.

It is...

-   little endian
-   64-bit
-   minimal, but comfortable (writing programs shouldn't be painful)

There should be a single-instruction way to do most things you might want, like clearing a
register, doing some maths (add, subtract, division, multiplation, exponents, etc), writing
to the "display", and much more.

It has (or will have) support for...

-   scheduling
-   interrupts
-   threading
-   extensible system bus for...
    -   coprocessing
    -   networking
    -   file io
-   maybe some other feature if you open a PR I like :)

Inspired (at least in spirit) by [Sol](https://rsms.me/sol-a-sunny-little-virtual-machine)
