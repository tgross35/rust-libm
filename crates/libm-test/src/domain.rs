#![allow(unused)]

use crate::Float;
use std::ops::Bound;

pub trait Domain<T: Copy + 'static> {
    /// The region for which the function is defined. Ignores poles.
    const DEFINED: (Bound<T>, Bound<T>);

    /// The region, if any, for which the function repeats. Used to test within.
    const PERIODIC: Option<(Bound<T>, Bound<T>)> = None;

    fn defined_asymptotes() -> impl Iterator<Item = (T, T)> {
        std::iter::empty()
    }

    /// Check if an input is a pole or branch point.
    fn is_pole(_input: T) -> bool {
        false
    }

    /// Points to check closer around, often zeros of the derivative.
    fn check_points() -> impl Iterator<Item = T> {
        std::iter::empty()
    }

    fn nan_handling(input: T) -> T {
        input
    }
}

/// Use for anything basic, no bounds, no asymptotes, etc.
pub struct Unbounded;

impl<F: Float> Domain<F> for Unbounded {
    const DEFINED: (Bound<F>, Bound<F>) = unbounded();
}

pub mod asin {
    use super::*;
    pub struct D;

    impl<F: Float> Domain<F> for D {
        const DEFINED: (Bound<F>, Bound<F>) =
            (Bound::Included(F::NEG_ONE), Bound::Included(F::ONE));
    }
}

pub mod acos {
    use super::*;
    pub struct D;

    impl<F: Float> Domain<F> for D {
        const DEFINED: (Bound<F>, Bound<F>) =
            (Bound::Included(F::NEG_ONE), Bound::Included(F::ONE));
    }
}

pub mod atan {
    pub use super::Unbounded as D;
}

pub mod asinh {
    pub use super::Unbounded as D;
}

pub mod acosh {
    use super::*;
    pub struct D;

    impl<F: Float> Domain<F> for D {
        const DEFINED: (Bound<F>, Bound<F>) = (Bound::Included(F::ONE), Bound::Unbounded);
    }
}

pub mod atanh {
    use super::*;
    pub struct D;

    impl<F: Float> Domain<F> for D {
        const DEFINED: (Bound<F>, Bound<F>) =
            (Bound::Excluded(F::NEG_ONE), Bound::Excluded(F::ONE));

        fn defined_asymptotes() -> impl Iterator<Item = (F, F)> {
            [(F::NEG_ONE, F::CONSTS.neg_inf), (F::ONE, F::CONSTS.inf)].into_iter()
        }
    }
}

pub mod sin {
    use super::*;
    pub struct D;

    impl<F: Float> Domain<F> for D {
        const DEFINED: (Bound<F>, Bound<F>) = unbounded();

        const PERIODIC: Option<(Bound<F>, Bound<F>)> = Some((
            Bound::Excluded(F::CONSTS.neg_pi),
            Bound::Included(F::CONSTS.pi),
        ));
    }
}

pub mod cos {
    pub use super::sin::D;
}

pub mod tan {
    pub use super::Unbounded as D;
}

pub mod cosh {
    pub use super::Unbounded as D;
}

pub mod sinh {
    pub use super::Unbounded as D;
}

pub mod tanh {
    pub use super::Unbounded as D;
}

pub mod cbrt {
    pub use super::Unbounded as D;
}

pub mod ceil {
    pub use super::Unbounded as D;
}

pub mod floor {
    pub use super::Unbounded as D;
}

pub mod erf {
    pub use super::Unbounded as D;
}

pub mod exp {
    pub use super::Unbounded as D;
}

pub mod exp10 {
    pub use super::exp::D;
}

pub mod exp2 {
    pub use super::exp::D;
}

pub mod expm1 {
    pub use super::Unbounded as D;
}

pub mod fabs {
    pub use super::Unbounded as D;
}

pub mod frexp {
    pub use super::Unbounded as D;
}

pub mod j0 {
    pub use super::Unbounded as D;
}

pub mod j1 {
    pub use super::Unbounded as D;
}

pub mod lgamma {}
pub mod lgamma_r {}

pub mod log {
    use super::*;
    pub struct D;

    impl<F: Float> Domain<F> for D {
        const DEFINED: (Bound<F>, Bound<F>) = strictly_positive();

        fn defined_asymptotes() -> impl Iterator<Item = (F, F)> {
            [(F::ZERO, F::CONSTS.neg_inf)].into_iter()
        }
    }
}

pub mod log10 {
    pub use super::log::D;
}

pub mod log1p {}

pub mod log2 {
    pub use super::log::D;
}

pub mod modf {}
pub mod nextafter {}
pub mod rint {
    pub use super::log::D;
}
pub mod round {
    pub use super::Unbounded as D;
}
pub mod sincos {
    pub use super::sin::D;
}

pub mod sqrt {}
pub mod tgamma {}

pub mod trunc {
    pub use super::Unbounded as D;
}

pub mod atan2 {}
pub mod copysign {}
pub mod fdim {}
pub mod fma {}
pub mod fmax {}
pub mod fmin {}
pub mod fmod {}
pub mod hypot {}
pub mod ilogb {}
pub mod jn {}
pub mod ldexp {}
pub mod pow {}
pub mod remainder {}
pub mod remquo {}
pub mod scalbn {}

/// x ∈ ℝ
const fn unbounded<F: Float>() -> (Bound<F>, Bound<F>) {
    (Bound::Unbounded, Bound::Unbounded)
}

/// x ∈ ℝ > 0
const fn strictly_positive<F: Float>() -> (Bound<F>, Bound<F>) {
    (Bound::Excluded(F::ZERO), Bound::Unbounded)
}
