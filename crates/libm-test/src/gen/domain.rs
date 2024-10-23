/*
Does the following:

- Check 100 values near each of the bounds
- If there are defined asymptotes, check those
- Figure out a number of tests to do within the domain. If exhaustive, check all (but skip
  NaNs?)
- Check near zero if defined
- If unbounded, ensure that real inputs do not produce any NaNs
- If periodic, check that results are identical for a few periods (?)


*/

#![allow(unused)]

use crate::{domain::Domain, Float};
use std::ops::{Bound, RangeBounds};

pub fn near_bounds<F: Float, D: Domain<F>>() -> Vec<F> {
    let mut v = Vec::new();

    let lower = D::DEFINED.0;
    let upper = D::DEFINED.1;

    if let (Bound::Included(l) | Bound::Excluded(l), Bound::Included(u) | Bound::Excluded(u)) =
        (lower, upper)
    {
        assert!(l < u, "lower bound must be less than upper bound");
    }

    let validate_bound = |b: Bound<F>| match b {
        Bound::Included(v) | Bound::Excluded(v) => {
            assert!(!v.is_nan());
            assert!(!v.is_infinite());
        }
        Bound::Unbounded => (),
    };

    validate_bound(lower);
    validate_bound(upper);

    v.retain(|f| D::DEFINED.contains(f));

    // No NaNs should be contained
    v.sort_by(|a, b| a.total_cmp(b));

    // let exceeds_upper = |v: F| match upper {
    //     Bound::Included(_) => todo!(),
    //     Bound::Excluded(_) => todo!(),
    //     Bound::Unbounded => todo!(),
    // };

    // // step through lower bounds as long as it doesn't exceed upper
    // // TODO: implement `next_up`, `next_down` on `Float`. Assert not NaN.

    // match (lower, upper) {
    //     (Bound::Included(_), Bound::Included(_)) => todo!(),
    //     (Bound::Included(_), Bound::Excluded(_)) => todo!(),
    //     (Bound::Included(_), Bound::Unbounded) => todo!(),
    //     (Bound::Excluded(_), Bound::Included(_)) => todo!(),
    //     (Bound::Excluded(_), Bound::Excluded(_)) => todo!(),
    //     (Bound::Excluded(_), Bound::Unbounded) => todo!(),
    //     (Bound::Unbounded, Bound::Included(_)) => todo!(),
    //     (Bound::Unbounded, Bound::Excluded(_)) => todo!(),
    //     (Bound::Unbounded, Bound::Unbounded) => todo!(),
    // }

    v
}

pub fn near_asymptotes<F: Float, D: Domain<F>>() {}
