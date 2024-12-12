//! A generator that checks a handful of cases near infinities, zeros, asymptotes, and NaNs.

use libm::support::Float;

use crate::domain::{Domain, HasDomain};
use crate::{FloatExt, MathOp};

/// Number of values near an interesting point to check.
// FIXME(ntests): replace this with a more logical algorithm
const AROUND: usize = 100;

/// Some functions have infinite asymptotes, limit how many we check.
const MAX_ASYMPTOTES: usize = 10;

/// Create a test case iterator.
pub fn get_test_cases<Op>() -> impl Iterator<Item = (Op::FTy,)>
where
    Op: MathOp + HasDomain<Op::FTy>,
{
    // Create a vector full of values near interesting (bounds, asymptotes, etc).
    let mut values = Vec::new();
    generate_near_limits::<Op::FTy>(&mut values);
    generate_near_asymptotes::<Op::FTy, Op::D>(&mut values);
    values.sort_by_key(|x| x.to_bits());
    values.dedup_by_key(|x| x.to_bits());
    values.into_iter().map(|v| (v,))
}

fn generate_near_limits<F: Float>(values: &mut Vec<F>) {
    values.push(F::MIN);
    values.push(F::MAX);

    // Values around min and max are interesting
    count_up(F::MIN, values);
    count_down(F::MAX, values);
    values.push(F::INFINITY);
    values.push(F::NEG_INFINITY);
    values.push(F::NEG_ZERO);

    // Check some special values that aren't included in the above ranges
    values.push(F::NAN);
    values.extend(F::consts().iter());

    count_up(F::ONE, values);
    count_up(F::ZERO, values);
    count_up(F::NEG_ONE, values);
    count_down(F::ONE, values);
    count_down(F::ZERO, values);
    count_down(F::NEG_ONE, values);
}

fn generate_near_asymptotes<F: Float, D: Domain<F>>(values: &mut Vec<F>) {
    // Check around asymptotest
    for (from, _to) in D::defined_asymptotes().take(MAX_ASYMPTOTES) {
        count_up(from, values);
        count_down(from, values);
    }

    for x in D::check_points().take(MAX_ASYMPTOTES) {
        count_up(x, values);
        count_down(x, values);
    }
}

/// Iterator that increments ULP up.
fn count_up<F: Float>(mut x: F, values: &mut Vec<F>) {
    assert!(!x.is_nan());
    assert!(!x.is_infinite());

    let mut count = 0;
    while !x.is_infinite() && count < AROUND {
        values.push(x);
        x = x.next_up();
        count += 1;
    }
}

/// Iterator that increments ULP down.
fn count_down<F: Float>(mut x: F, values: &mut Vec<F>) {
    assert!(!x.is_nan());
    assert!(!x.is_infinite());

    let mut count = 0;
    while !x.is_infinite() && count < AROUND {
        values.push(x);
        x = x.next_down();
        count += 1;
    }
}
