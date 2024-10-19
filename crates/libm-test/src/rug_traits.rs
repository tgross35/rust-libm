// pub trait Construct

// f32 -> f32
// (f32, f32) -> f32

// fn map() {

// }

use rug::Assign;

use crate::{CheckOutput, TupleCall};

// pub trait Helper<Func> {

// // }
// pub struct RugHelper<T> {}

// fn overall(existing: &mut (rug::Float,), op: fn(&mut rug::Float) -> f32) {}

// (Float) -> Float
// (Float, Float) -> Float
// (Float, i32) -> Float

pub trait Thing<RugTy> {
    fn set_values(self, dst: &mut RugTy);
}

impl Thing<(rug::Float,)> for (f32,) {
    fn set_values(self, dst: &mut (rug::Float,)) {
        dst.0.assign(self.0);
    }
}

impl Thing<(rug::Float,)> for (f64,) {
    fn set_values(self, dst: &mut (rug::Float,)) {
        dst.0.assign(self.0);
    }
}

impl TupleCall<fn(rug::Float) -> rug::Float> for (rug::Float,) {
    type Output = (rug::Float,);

    fn call(self, f: fn(rug::Float) -> rug::Float) -> Self::Output {
        (f(self.0),)
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

impl ToSomething<f64> for (rug::Float,) {
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
