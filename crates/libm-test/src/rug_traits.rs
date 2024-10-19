// pub trait Construct

// f32 -> f32
// (f32, f32) -> f32

// fn map() {

// }

use rug::Assign;

use crate::TupleCall;

pub trait TupleAssign<RugTy> {
    fn new_mpfloat(prec: u32) -> RugTy;
    fn set_values(self, dst: &mut RugTy);
}

impl TupleAssign<(rug::Float,)> for (f32,) {
    fn new_mpfloat(prec: u32) -> (rug::Float,) {
        (rug::Float::new(prec),)
    }

    fn set_values(self, dst: &mut (rug::Float,)) {
        dst.0.assign(self.0);
    }
}

impl TupleAssign<(rug::Float, rug::Float)> for (f32, f32) {
    fn new_mpfloat(prec: u32) -> (rug::Float, rug::Float) {
        (rug::Float::new(prec), rug::Float::new(prec))
    }

    fn set_values(self, dst: &mut (rug::Float, rug::Float)) {
        dst.0.assign(self.0);
        dst.1.assign(self.1);
    }
}

impl TupleAssign<(rug::Float,)> for (f64,) {
    fn new_mpfloat(prec: u32) -> (rug::Float,) {
        (rug::Float::new(prec),)
    }

    fn set_values(self, dst: &mut (rug::Float,)) {
        dst.0.assign(self.0);
    }
}

impl TupleAssign<(rug::Float, rug::Float)> for (f64, f64) {
    fn new_mpfloat(prec: u32) -> (rug::Float, rug::Float) {
        (rug::Float::new(prec), rug::Float::new(prec))
    }

    fn set_values(self, dst: &mut (rug::Float, rug::Float)) {
        dst.0.assign(self.0);
        dst.1.assign(self.1);
    }
}

impl TupleCall<fn(rug::Float) -> rug::Float> for (rug::Float,) {
    type Output = (rug::Float,);

    fn call(self, f: fn(rug::Float) -> rug::Float) -> Self::Output {
        (f(self.0),)
    }
}

// e.g. `atan2`
impl TupleCall<fn(rug::Float, &rug::Float) -> rug::Float> for (rug::Float, rug::Float) {
    type Output = (rug::Float, rug::Float);

    fn call(self, f: fn(rug::Float, &rug::Float) -> rug::Float) -> Self::Output {
        (f(self.0, &self.1), self.1)
    }
}

pub trait ToSomething<Dest> {
    fn do_thing(&self) -> Dest;
}

impl ToSomething<f32> for (rug::Float,) {
    fn do_thing(&self) -> f32 {
        self.0.to_f32()
    }
}
impl ToSomething<f32> for (rug::Float, rug::Float) {
    fn do_thing(&self) -> f32 {
        self.0.to_f32()
    }
}

impl ToSomething<f64> for (rug::Float,) {
    fn do_thing(&self) -> f64 {
        self.0.to_f64()
    }
}
impl ToSomething<f64> for (rug::Float, rug::Float) {
    fn do_thing(&self) -> f64 {
        self.0.to_f64()
    }
}

// impl TupleCall<fn(&mut rug::Float)> for &mut (rug::Float,) {
//     type Output = ();

//     fn call(self, f: fn(&mut rug::Float)) -> Self::Output {
//         f(&mut self.0)
//     }
// }

// impl TupleCall<fn(&mut rug::Float)> for &mut (rug::Float,) {
//     type Output = ();

//     fn call(self, f: fn(&mut rug::Float)) -> Self::Output {
//         f(&mut self.0)
//     }
// }
