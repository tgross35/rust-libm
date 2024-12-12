//! Traits and operations related to bounds of a function.

use std::fmt;
use std::ops::{self, Bound};

use crate::Float;

/// A trait to be implemented on types representing a function's domain.
///
/// Since multiple functions share the same domain, this doesn't get implemented on the `op::*`
/// type. Instead, this gets applied to a new unit struct, then the `op::*` type should
/// implement `HasDomain`.
pub trait Domain<T>
where
    T: Copy + fmt::Debug + ops::Add<Output = T> + ops::Sub<Output = T> + PartialOrd + 'static,
{
    /// The region for which the function is defined. Ignores poles.
    const DEFINED: (Bound<T>, Bound<T>);

    /// Additional points to check closer around. These can be e.g. undefined asymptotes or
    /// inflection points.
    ///
    /// -inf, -1, 0, 1, and inf always get checked so may be excluded.
    fn check_points() -> impl Iterator<Item = T> {
        std::iter::empty()
    }
}

/* Possible domains */

/// Use for anything basic, no bounds, no asymptotes, etc.
pub struct Unbounded;
impl<F: Float> Domain<F> for Unbounded {
    const DEFINED: (Bound<F>, Bound<F>) = unbounded();
}

/// Used for versions of `asin` and `acos`.
pub struct InvTrigPeriodic;
impl<F: Float> Domain<F> for InvTrigPeriodic {
    const DEFINED: (Bound<F>, Bound<F>) = (Bound::Included(F::NEG_ONE), Bound::Included(F::ONE));
}

/// Domain for `acosh`
pub struct ACoshDomain;
impl<F: Float> Domain<F> for ACoshDomain {
    const DEFINED: (Bound<F>, Bound<F>) = (Bound::Included(F::ONE), Bound::Unbounded);
}

/// Domain for `atanh`
pub struct ATanhDomain;
impl<F: Float> Domain<F> for ATanhDomain {
    const DEFINED: (Bound<F>, Bound<F>) = (Bound::Excluded(F::NEG_ONE), Bound::Excluded(F::ONE));
}

/// Domain for `sin`, `cos`, and `tan`
pub struct TrigDomain;
impl<F: Float> Domain<F> for TrigDomain {
    const DEFINED: (Bound<F>, Bound<F>) = unbounded();

    fn check_points() -> impl Iterator<Item = F> {
        [-F::PI, -F::FRAC_PI_2, F::FRAC_PI_2, F::PI].into_iter()
    }
}

/// Domain for `log` in various bases
pub struct LogDomain;
impl<F: Float> Domain<F> for LogDomain {
    const DEFINED: (Bound<F>, Bound<F>) = strictly_positive();
}

/// Domain for `log1p` i.e. `log(1 + x)`
pub struct Log1pDomain;
impl<F: Float> Domain<F> for Log1pDomain {
    const DEFINED: (Bound<F>, Bound<F>) = (Bound::Excluded(F::NEG_ONE), Bound::Unbounded);
}

/// Domain for `sqrt`
pub struct SqrtDomain;
impl<F: Float> Domain<F> for SqrtDomain {
    const DEFINED: (Bound<F>, Bound<F>) = positive();
}

/// x ∈ ℝ
const fn unbounded<F: Float>() -> (Bound<F>, Bound<F>) {
    (Bound::Unbounded, Bound::Unbounded)
}

/// x ∈ ℝ >= 0
const fn positive<F: Float>() -> (Bound<F>, Bound<F>) {
    (Bound::Included(F::ZERO), Bound::Unbounded)
}

/// x ∈ ℝ > 0
const fn strictly_positive<F: Float>() -> (Bound<F>, Bound<F>) {
    (Bound::Excluded(F::ZERO), Bound::Unbounded)
}

/// Implement on `op::*` types to indicate how they are bounded.
pub trait HasDomain<T>
where
    T: Copy + fmt::Debug + ops::Add<Output = T> + ops::Sub<Output = T> + PartialOrd + 'static,
{
    type D: Domain<T>;
}

/// Implement [`HasDomain`] for both the `f32` and `f64` variants of a function.
macro_rules! impl_has_domain {
    ($($fn_name:ident => $domain:ty;)*) => {
        paste::paste! {
            $(
                // Implement for f64 functions
                impl HasDomain<f64> for $crate::op::$fn_name::Routine {
                    type D = $domain;
                }

                // Implement for f32 functions
                impl HasDomain<f32> for $crate::op::[< $fn_name f >]::Routine {
                    type D = $domain;
                }
            )*
        }
    };
}

// Tie functions together with their domains.
impl_has_domain! {
    acos => InvTrigPeriodic;
    acosh => ACoshDomain;
    asin => InvTrigPeriodic;
    asinh => Unbounded;
    // TODO asymptotes
    atan => Unbounded;
    atanh => ATanhDomain;
    cbrt => Unbounded;
    ceil => Unbounded;
    cos => TrigDomain;
    cosh => Unbounded;
    erf => Unbounded;
    exp => Unbounded;
    exp10 => Unbounded;
    exp2 => Unbounded;
    expm1 => Unbounded;
    fabs => Unbounded;
    floor => Unbounded;
    frexp => Unbounded;
    j0 => Unbounded;
    j1 => Unbounded;
    log => LogDomain;
    log10 => LogDomain;
    log1p => Log1pDomain;
    log2 => LogDomain;
    rint => Unbounded;
    round => Unbounded;
    sin => TrigDomain;
    sinh => Unbounded;
    sqrt => SqrtDomain;
    tan => TrigDomain;
    tanh => Unbounded;
    trunc => Unbounded;
}
