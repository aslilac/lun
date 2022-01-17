In general the structure of the impl names is `{opname}_{datatype}_{operands}`

-   datatype is usually one of...

    -   i8
    -   u8
    -   i16
    -   u16
    -   i32
    -   u32
    -   i64
    -   u64
    -   b (byte, 8 bits)
    -   q (quarter-word, 16 bits)
    -   h (half-word, 32 bits)
    -   w (word, 64 bits)

-   operand is usually one of...

    -   i (immediate value, encoded in the instruction)
    -   r (register)

## Instructions

### `add`

-   [x] `add_i8_rir`
-   [x] `add_i8_rrr`
-   [x] `add_u8_rir`
-   [x] `add_u8_rrr`
-   [x] `add_i16_rir`
-   [x] `add_i16_rrr`
-   [x] `add_u16_rir`
-   [x] `add_u16_rrr`
-   [x] `add_f32_rir`
-   [x] `add_f32_rrr`
-   [x] `add_i32_rir`
-   [x] `add_i32_rrr`
-   [x] `add_u32_rir`
-   [x] `add_u32_rrr`
-   [x] `add_f64_rir`
-   [x] `add_f64_rrr`
-   [x] `add_i64_rir`
-   [x] `add_i64_rrr`
-   [x] `add_u64_rir`
-   [x] `add_u64_rrr`

### `and`

-   [x] `and_b_rrr`
-   [x] `and_q_rrr`
-   [x] `and_h_rrr`
-   [x] `and_w_rrr`

### `br`

-   [x] `br_x_i`

### `cmp`

-   [ ] `cmp_b_r`
-   [ ] `cmp_b_rr`
-   [ ] `cmp_q_r`
-   [ ] `cmp_q_rr`
-   [ ] `cmp_h_r`
-   [ ] `cmp_h_rr`
-   [x] `cmp_w_r`
-   [x] `cmp_w_rr`

### `div`

-   [ ] `div_i8_rirr`
-   [ ] `div_i8_rrrr`
-   [ ] `div_u8_rirr`
-   [ ] `div_u8_rrrr`
-   [ ] `div_i16_rirr`
-   [ ] `div_i16_rrrr`
-   [ ] `div_u16_rirr`
-   [ ] `div_u16_rrrr`
-   [ ] `div_f32_rir`
-   [ ] `div_f32_rrr`
-   [ ] `div_i32_rirr`
-   [ ] `div_i32_rrrr`
-   [ ] `div_u32_rirr`
-   [ ] `div_u32_rrrr`
-   [ ] `div_f64_rir`
-   [ ] `div_f64_rrr`
-   [x] `div_i64_rirr`
-   [x] `div_i64_rrrr`
-   [ ] `div_u64_rirr`
-   [ ] `div_u64_rrrr`

### `exp`

-   [ ] `exp_i8_rir`
-   [ ] `exp_i8_rrr`
-   [ ] `exp_u8_rir`
-   [ ] `exp_u8_rrr`
-   [ ] `exp_i16_rir`
-   [ ] `exp_i16_rrr`
-   [ ] `exp_u16_rir`
-   [ ] `exp_u16_rrr`
-   [ ] `exp_f32_rir`
-   [ ] `exp_f32_rrr`
-   [ ] `exp_i32_rir`
-   [ ] `exp_i32_rrr`
-   [ ] `exp_u32_rir`
-   [ ] `exp_u32_rrr`
-   [ ] `exp_f64_rir`
-   [ ] `exp_f64_rrr`
-   [x] `exp_i64_rir`
-   [x] `exp_i64_rrr`
-   [x] `exp_u64_rir`
-   [x] `exp_u64_rrr`

### `halt`

-   [ ] `halt_x_x`

### `mul`

-   [ ] `mul_i8_rir`
-   [ ] `mul_i8_rrr`
-   [ ] `mul_u8_rir`
-   [ ] `mul_u8_rrr`
-   [ ] `mul_i16_rir`
-   [ ] `mul_i16_rrr`
-   [ ] `mul_u16_rir`
-   [ ] `mul_u16_rrr`
-   [ ] `mul_f32_rir`
-   [ ] `mul_f32_rrr`
-   [ ] `mul_i32_rir`
-   [ ] `mul_i32_rrr`
-   [ ] `mul_u32_rir`
-   [ ] `mul_u32_rrr`
-   [ ] `mul_f64_rir`
-   [ ] `mul_f64_rrr`
-   [x] `mul_i64_rir`
-   [x] `mul_i64_rrr`
-   [x] `mul_u64_rir`
-   [x] `mul_u64_rrr`

### `mv`

-   [x] `mv_b_rr`
-   [x] `mv_q_rr`
-   [x] `mv_h_rr`
-   [x] `mv_w_rr`

### `not`

-   [ ] `not_b_rr`
-   [ ] `not_q_rr`
-   [ ] `not_h_rr`
-   [ ] `not_w_rr`

### `or`

-   [x] `or_b_rrr`
-   [x] `or_q_rrr`
-   [x] `or_h_rrr`
-   [x] `or_w_rrr`

### `pop`

-   [x] `pop_w_r`

### `push`

-   [x] `push_w_r`

### `put`

-   [x] `put_b_i`
-   [x] `put_b_r`
-   [x] `put_w_r`

### `slp`

-   [ ] `slp`

### `sl`

-   [ ] `sl_b_rir`
-   [ ] `sl_b_rrr`
-   [ ] `sl_q_rir`
-   [ ] `sl_q_rrr`
-   [ ] `sl_h_rir`
-   [ ] `sl_h_rrr`
-   [ ] `sl_w_rir`
-   [ ] `sl_w_rrr`

### `sr`

-   [ ] `sr_b_rir`
-   [ ] `sr_b_rrr`
-   [ ] `sr_q_rir`
-   [ ] `sr_q_rrr`
-   [ ] `sr_h_rir`
-   [ ] `sr_h_rrr`
-   [ ] `sr_w_rir`
-   [ ] `sr_w_rrr`

### `ser`

-   [ ] `ser_b_rir`
-   [ ] `ser_b_rrr`
-   [ ] `ser_q_rir`
-   [ ] `ser_q_rrr`
-   [ ] `ser_h_rir`
-   [ ] `ser_h_rrr`
-   [ ] `ser_w_rir`
-   [ ] `ser_w_rrr`

### `sub`

-   [ ] `sub_i8_rir`
-   [ ] `sub_i8_rrr`
-   [ ] `sub_u8_rir`
-   [ ] `sub_u8_rrr`
-   [ ] `sub_i16_rir`
-   [ ] `sub_i16_rrr`
-   [ ] `sub_u16_rir`
-   [ ] `sub_u16_rrr`
-   [ ] `sub_f32_rir`
-   [ ] `sub_f32_rrr`
-   [ ] `sub_i32_rir`
-   [ ] `sub_i32_rrr`
-   [ ] `sub_u32_rir`
-   [ ] `sub_u32_rrr`
-   [ ] `sub_f64_rir`
-   [ ] `sub_f64_rrr`
-   [ ] `sub_i64_rir`
-   [ ] `sub_i64_rrr`
-   [ ] `sub_u64_rir`
-   [ ] `sub_u64_rrr`

### `xor`

-   [x] `xor_b_r`
-   [x] `xor_b_rrr`
-   [x] `xor_q_r`
-   [x] `xor_q_rrr`
-   [x] `xor_h_r`
-   [x] `xor_h_rrr`
-   [x] `xor_w_r`
-   [x] `xor_w_rrr`
