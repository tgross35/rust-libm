//! Traits related to testing.
//!
//! There are three main traits in this module:
//!
//! - `GenerateInput`: implemented on any types that create test cases.
//! - `TupleCall`: implemented on tuples to allow calling them as function arguments.
//! - `CheckOutput`: implemented on anything that is an output type for validation against an
//!   expected value.

use crate::{Float, Hex, Int};
use std::fmt;

/// Implement this on types that can generate a sequence of tuples for test input.
pub trait GenerateInput<TupleArgs> {
    fn get_cases(&self) -> impl ExactSizeIterator<Item = TupleArgs>;
}

/// Trait for calling a function with a tuple as arguments.
///
/// Implemented on the tuple with the function signature as the generic (so we can use the same
/// tuple for multiple signatures).
pub trait TupleCall<Func>: fmt::Debug {
    type Output;
    fn call(self, f: Func) -> Self::Output;
}

/// A trait to implement on any output type so we can verify it in a generic way.
pub trait CheckOutput<Input>: Sized {
    /// Assert that `self` and `expected` are the same.
    ///
    /// `input` is only used here for error messages.
    fn validate(self, expected: Self, input: Input, allowed_ulp: u32);
}

impl<T1, R> TupleCall<fn(T1) -> R> for (T1,)
where
    T1: fmt::Debug,
{
    type Output = R;

    fn call(self, f: fn(T1) -> R) -> Self::Output {
        f(self.0)
    }
}

impl<T1, T2, R> TupleCall<fn(T1, T2) -> R> for (T1, T2)
where
    T1: fmt::Debug,
    T2: fmt::Debug,
{
    type Output = R;

    fn call(self, f: fn(T1, T2) -> R) -> Self::Output {
        f(self.0, self.1)
    }
}

impl<T1, T2, R> TupleCall<fn(T1, &mut T2) -> R> for (T1,)
where
    T1: fmt::Debug,
    T2: fmt::Debug + Default,
{
    type Output = (R, T2);

    fn call(self, f: fn(T1, &mut T2) -> R) -> Self::Output {
        let mut t2 = T2::default();
        (f(self.0, &mut t2), t2)
    }
}

impl<T1, T2, T3, R> TupleCall<fn(T1, T2, T3) -> R> for (T1, T2, T3)
where
    T1: fmt::Debug,
    T2: fmt::Debug,
    T3: fmt::Debug,
{
    type Output = R;

    fn call(self, f: fn(T1, T2, T3) -> R) -> Self::Output {
        f(self.0, self.1, self.2)
    }
}

impl<T1, T2, T3, R> TupleCall<fn(T1, T2, &mut T3) -> R> for (T1, T2)
where
    T1: fmt::Debug,
    T2: fmt::Debug,
    T3: fmt::Debug + Default,
{
    type Output = (R, T3);

    fn call(self, f: fn(T1, T2, &mut T3) -> R) -> Self::Output {
        let mut t3 = T3::default();
        (f(self.0, self.1, &mut t3), t3)
    }
}

impl<T1, T2, T3> TupleCall<fn(T1, &mut T2, &mut T3)> for (T1,)
where
    T1: fmt::Debug,
    T2: fmt::Debug + Default,
    T3: fmt::Debug + Default,
{
    type Output = (T2, T3);

    fn call(self, f: fn(T1, &mut T2, &mut T3)) -> Self::Output {
        let mut t2 = T2::default();
        let mut t3 = T3::default();
        f(self.0, &mut t2, &mut t3);
        (t2, t3)
    }
}

// Implement for floats
impl<F, Input> CheckOutput<Input> for F
where
    F: Float + Hex,
    Input: Hex + fmt::Debug,
    u32: TryFrom<F::SignedInt, Error: fmt::Debug>,
{
    fn validate(self, expected: Self, input: Input, allowed_ulp: u32) {
        let make_msg = || {
            format!(
                "expected {expected:?} crate {self:?} ({expbits}, {actbits}) input {input:?} ({ibits})",
                expbits = expected.hex(),
                actbits = self.hex(),
                ibits = input.hex()
           )
        };

        // Check when both are NaN
        if self.is_nan() && expected.is_nan() {
            assert_eq!(
                self.to_bits(),
                expected.to_bits(),
                "NaN have different bitpatterns: {}",
                make_msg()
            );
            // Nothing else to check
            return;
        } else if self.is_nan() || expected.is_nan() {
            panic!("mismatched NaN: {}", make_msg());
        }

        // Make sure that the signs are the same before checing ULP
        assert_eq!(
            self.signum(),
            expected.signum(),
            "mismatched signs: {}",
            make_msg()
        );

        let ulp_diff = self
            .to_bits()
            .signed()
            .checked_sub(expected.to_bits().signed())
            .unwrap()
            .abs();

        let ulp_u32 = u32::try_from(ulp_diff).unwrap_or_else(|e| {
            panic!("{e:?}: ulp of {ulp_diff} exceeds u32::MAX: {}", make_msg())
        });

        assert!(
            ulp_u32 <= allowed_ulp,
            "ulp {ulp_diff} > {allowed_ulp}: {}",
            make_msg()
        );
    }
}

/// Implement `CheckOutput` for combinations of types.
macro_rules! impl_tuples {
    ($(($a:ty, $b:ty);)*) => {
        $(
            impl<Input: Hex + fmt::Debug> CheckOutput<Input> for ($a, $b) {
                fn validate(self, expected: Self, input: Input, allowed_ulp: u32)
                {
                    self.0.validate(expected.0, input, allowed_ulp);
                    self.1.validate(expected.1, input, allowed_ulp);
                }
            }
        )*
    };
}

impl_tuples!(
    (f32, i32);
    (f64, i32);
    (f32, f32);
    (f64, f64);
);
