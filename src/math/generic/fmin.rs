/* SPDX-License-Identifier: MIT OR Apache-2.0 */
//! IEEE 754-2008 `minNum`. This has been superseded by IEEE 754-2019 `minimumNumber`.
//!
//! Per the spec, returns the canonicalized result of:
//! - `x` if `x < y`
//! - `y` if `y < x`
//! - The other number if one is NaN
//! - Otherwise, either `x` or `y`, canonicalized
//! - -0.0 and +0.0 may be disregarded (unlike newer operations)
//!
//! Excluded from our implementation is sNaN handling.
//!
//! More on the differences: [link].
//!
//! [link]: https://grouper.ieee.org/groups/msc/ANSI_IEEE-Std-754-2019/background/minNum_maxNum_Removal_Demotion_v3.pdf

use super::super::Float;

pub fn fmin<F: Float>(x: F, y: F) -> F {
    let res = if y.is_nan() || x < y { x } else { y };
    // Canonicalize
    res * F::ONE
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::support::{Hexf, Int};

    fn spec_test<F: Float>() {
        let cases = [
            (F::ZERO, F::ZERO, F::ZERO),
            (F::ONE, F::ONE, F::ONE),
            (F::ZERO, F::ONE, F::ZERO),
            (F::ONE, F::ZERO, F::ZERO),
            (F::ZERO, F::NEG_ONE, F::NEG_ONE),
            (F::NEG_ONE, F::ZERO, F::NEG_ONE),
            (F::INFINITY, F::ZERO, F::ZERO),
            (F::NEG_INFINITY, F::ZERO, F::NEG_INFINITY),
            (F::NAN, F::ZERO, F::ZERO),
            (F::ZERO, F::NAN, F::ZERO),
            (F::NAN, F::NAN, F::NAN),
        ];

        for (x, y, res) in cases {
            let val = fmin(x, y);
            assert_biteq!(val, res, "fmin({}, {})", Hexf(x), Hexf(y));
        }
    }

    #[test]
    #[cfg(f16_enabled)]
    fn spec_tests_f16() {
        spec_test::<f16>();
    }

    #[test]
    fn spec_tests_f32() {
        spec_test::<f32>();
    }

    #[test]
    fn spec_tests_f64() {
        spec_test::<f64>();
    }

    #[test]
    #[cfg(f128_enabled)]
    fn spec_tests_f128() {
        spec_test::<f128>();
    }
}
