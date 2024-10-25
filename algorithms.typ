= Introduction



- $m_a$: the mantissa of $a$, in base 2
- $p_a$: the exponent of $a$, in base 2. I.e. $a = m_a * 2^(p_a)$
- `uqN` variable names (e.g. `uq1`): refers to Q notation for fixed-point
  numbers. `UQ1.31` is an unsigned fixed-point number with 1 integral bit  and 31
  decimal bits. A `uqN` variable of type `uM` will have N bits of integer and
  `M-N` bits of fraction (`UQN.(M-N)`).
- `hw`: half width, i.e. for `f64` this will be a `u32`.
- `x` is the best estimate of `1/m_b`


= Algorithms for `musl`


== `cbrt`: Cube Root

// Shift the number into a Q0

The polynomial:

$
c_0 + c_1 * x + c_2 * x^2 + c_3 * x^3 
$

approximates $x^(1/3)$ on $[1, 2]$ with maximal error < $9.2*10^(-5)$ (error
is at $x = 2$).
