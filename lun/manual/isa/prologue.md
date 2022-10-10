# Prologue

-   curly braces (`{` and `}`) are used to denote a fixed set of options that can be
    chosen from usually separated by commas
    -   `{a,b}` would mean: in this location, must either be "a" or be "b"
    -   `{a}` would mean: in this location, must be "a"
-   `numeric` represents `i8`, `u8`, `i16`, `u16`, `f32`, `i32`, `u32`, `f64`, `i64`, and `u64`
    -   often used as `{numeric}` to mean: in this location, must be one of the numeric data types
    -   an `f`, an `i`, or a `u` can also be used in place of `numeric` in places where the data
        size can be inferred (usually from the operands of an instruction)
-   `integer` represents `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, and `u64`
-   `float` represents `f32`, or `f64`
