use super::super::traits::{CastFrom, CastInto, Float, Int, MinInt};

pub trait Consts: Float {
    /// [1.0, 2^(1/3), 2^(2/3)]
    const ESCALE: [Self; 3];
    const C: &[Self];
}

impl Consts for f32 {
    const ESCALE: [Self; 3] = [1.0, f64::ESCALE[1] as f32, f64::ESCALE[2] as f32];

    const C: &[Self] = &[];
}

impl Consts for f64 {
    const ESCALE: [Self; 3] = [
        1.0,
        hf64!("0x1.428a2f98d728bp+0"), /* 2^(1/3) */
        hf64!("0x1.965fea53d6e3dp+0"), /* 2^(2/3) */
    ];
    const C: &[Self] = &[
        hf64!("0x1.1b0babccfef9cp-1"),
        hf64!("0x1.2c9a3e94d1da5p-1"),
        hf64!("-0x1.4dc30b1a1ddbap-3"),
        hf64!("0x1.7a8d3e4ec9b07p-6"),
    ];
}

pub fn cbrt<F>(x: F) -> F
where
    F: Float + Consts,
    F::Int: CastInto<u32>,
{
    let escale = F::ESCALE;
    let c = F::C;

    let hx = x.to_bits();
    let mut mant = x.frac();
    let sign = x.is_sign_negative();

    let mut e: u32 = (hx >> F::SIGNIFICAND_BITS).cast() & F::EXPONENT_MAX;

    // Exponent was saturated (infinite or NaN) or zero (zero or subnormal)
    if (e + 1) & F::EXPONENT_MAX < 2 {
        let ix = hx & !F::SIGN_MASK;
        if e == F::EXPONENT_MAX || ix == F::Int::ZERO {
            // Handle NaNs properly
            return x + x;
        }

        // Value is subnormal. Turn it into a normal number and adjust the
        // exponent.
        let nz = ix.leading_zeros() - F::EXPONENT_BITS;
        mant <<= nz;
        mant &= F::SIGNIFICAND_MASK;
        e -= nz - 1; // todo: wrapping?
    }

    // todo: what is this?
    e += 3072;

    todo!()
}
