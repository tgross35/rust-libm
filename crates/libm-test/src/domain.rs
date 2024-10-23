#![allow(unused)]

use crate::Float;
use std::ops::Bound;

pub trait Domain<T: Copy + 'static> {
    // type Input;
    // type Output;

    /// The region for which the function is defined. Ignores poles.
    const DEFINED: (Bound<T>, Bound<T>);

    /// The region, if any, for which the function repeats. Used to test within.
    const PERIODIC: Option<(Bound<T>, Bound<T>)> = None;

    /// Check if an input is a pole.
    fn is_pole(_input: T) -> bool {
        false
    }
}

pub mod cos {
    use super::*;
    pub struct D;

    impl<F: Float> Domain<F> for D {
        const DEFINED: (Bound<F>, Bound<F>) = (Bound::Unbounded, Bound::Unbounded);

        const PERIODIC: Option<(Bound<F>, Bound<F>)> = Some((
            Bound::Excluded(F::CONSTS.neg_pi),
            Bound::Included(F::CONSTS.pi),
        ));
    }
}
