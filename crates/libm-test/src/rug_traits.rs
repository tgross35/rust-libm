//! Traits for working with MPFR via `rug`
//!
//!

use rug::Assign;
pub use rug::Float as MpFloat;

use crate::TupleCall;

pub trait TupleAssign<RugTy> {
    fn new_mpfloat(prec: u32) -> RugTy;
    fn set_values(self, dst: &mut RugTy);
}

impl TupleAssign<(MpFloat,)> for (f32,) {
    fn new_mpfloat(prec: u32) -> (MpFloat,) {
        (MpFloat::new(prec),)
    }

    fn set_values(self, dst: &mut (MpFloat,)) {
        dst.0.assign(self.0);
    }
}

impl TupleAssign<(MpFloat, MpFloat)> for (f32, f32) {
    fn new_mpfloat(prec: u32) -> (MpFloat, MpFloat) {
        (MpFloat::new(prec), MpFloat::new(prec))
    }

    fn set_values(self, dst: &mut (MpFloat, MpFloat)) {
        dst.0.assign(self.0);
        dst.1.assign(self.1);
    }
}

impl TupleAssign<(i32, MpFloat)> for (i32, f32) {
    fn new_mpfloat(prec: u32) -> (i32, MpFloat) {
        (0, MpFloat::new(prec))
    }

    fn set_values(self, dst: &mut (i32, MpFloat)) {
        dst.0 = self.0;
        dst.1.assign(self.1);
    }
}

impl TupleAssign<(i32, MpFloat)> for (i32, f64) {
    fn new_mpfloat(prec: u32) -> (i32, MpFloat) {
        (0, MpFloat::new(prec))
    }

    fn set_values(self, dst: &mut (i32, MpFloat)) {
        dst.0 = self.0;
        dst.1.assign(self.1);
    }
}

impl TupleAssign<(MpFloat, MpFloat, MpFloat)> for (f32, f32, f32) {
    fn new_mpfloat(prec: u32) -> (MpFloat, MpFloat, MpFloat) {
        (MpFloat::new(prec), MpFloat::new(prec), MpFloat::new(prec))
    }

    fn set_values(self, dst: &mut (MpFloat, MpFloat, MpFloat)) {
        dst.0.assign(self.0);
        dst.1.assign(self.1);
        dst.2.assign(self.2);
    }
}

impl TupleAssign<(MpFloat,)> for (f64,) {
    fn new_mpfloat(prec: u32) -> (MpFloat,) {
        (MpFloat::new(prec),)
    }

    fn set_values(self, dst: &mut (MpFloat,)) {
        dst.0.assign(self.0);
    }
}

impl TupleAssign<(MpFloat, MpFloat)> for (f64, f64) {
    fn new_mpfloat(prec: u32) -> (MpFloat, MpFloat) {
        (MpFloat::new(prec), MpFloat::new(prec))
    }

    fn set_values(self, dst: &mut (MpFloat, MpFloat)) {
        dst.0.assign(self.0);
        dst.1.assign(self.1);
    }
}

impl TupleAssign<(MpFloat, MpFloat, MpFloat)> for (f64, f64, f64) {
    fn new_mpfloat(prec: u32) -> (MpFloat, MpFloat, MpFloat) {
        (MpFloat::new(prec), MpFloat::new(prec), MpFloat::new(prec))
    }

    fn set_values(self, dst: &mut (MpFloat, MpFloat, MpFloat)) {
        dst.0.assign(self.0);
        dst.1.assign(self.1);
        dst.2.assign(self.2);
    }
}

impl TupleCall<fn(MpFloat) -> MpFloat> for (MpFloat,) {
    type Output = (MpFloat,);

    fn call(self, f: fn(MpFloat) -> MpFloat) -> Self::Output {
        (f(self.0),)
    }
}

// e.g. `atan2`
impl TupleCall<fn(MpFloat, &MpFloat) -> MpFloat> for (MpFloat, MpFloat) {
    type Output = (MpFloat, MpFloat);

    fn call(self, f: fn(MpFloat, &MpFloat) -> MpFloat) -> Self::Output {
        (f(self.0, &self.1), self.1)
    }
}

// e.g. `jn`
impl TupleCall<fn(MpFloat, i32) -> MpFloat> for (i32, MpFloat) {
    type Output = (i32, MpFloat);

    fn call(self, f: fn(MpFloat, i32) -> MpFloat) -> Self::Output {
        (0, f(self.1, self.0))
    }
}

// e.g. manual fma
impl TupleCall<fn(MpFloat, &MpFloat, &MpFloat) -> MpFloat> for (MpFloat, MpFloat, MpFloat) {
    type Output = (MpFloat, MpFloat, MpFloat);

    fn call(self, f: fn(MpFloat, &MpFloat, &MpFloat) -> MpFloat) -> Self::Output {
        (f(self.0, &self.1, &self.2), self.1, self.2)
    }
}

pub trait ToSomething<Dest> {
    fn do_thing(&self) -> Dest;
}

impl ToSomething<f32> for (MpFloat,) {
    fn do_thing(&self) -> f32 {
        self.0.to_f32()
    }
}

impl ToSomething<f32> for (MpFloat, MpFloat) {
    fn do_thing(&self) -> f32 {
        self.0.to_f32()
    }
}

impl ToSomething<f32> for (i32, MpFloat) {
    fn do_thing(&self) -> f32 {
        self.1.to_f32()
    }
}
impl ToSomething<f64> for (i32, MpFloat) {
    fn do_thing(&self) -> f64 {
        self.1.to_f64()
    }
}

impl ToSomething<f32> for (MpFloat, MpFloat, MpFloat) {
    fn do_thing(&self) -> f32 {
        self.0.to_f32()
    }
}

impl ToSomething<f64> for (MpFloat,) {
    fn do_thing(&self) -> f64 {
        self.0.to_f64()
    }
}
impl ToSomething<f64> for (MpFloat, MpFloat) {
    fn do_thing(&self) -> f64 {
        self.0.to_f64()
    }
}
impl ToSomething<f64> for (MpFloat, MpFloat, MpFloat) {
    fn do_thing(&self) -> f64 {
        self.0.to_f64()
    }
}

// impl TupleCall<fn(&mut MpFloat)> for &mut (MpFloat,) {
//     type Output = ();

//     fn call(self, f: fn(&mut MpFloat)) -> Self::Output {
//         f(&mut self.0)
//     }
// }

// impl TupleCall<fn(&mut MpFloat)> for &mut (MpFloat,) {
//     type Output = ();

//     fn call(self, f: fn(&mut MpFloat)) -> Self::Output {
//         f(&mut self.0)
//     }
// }
