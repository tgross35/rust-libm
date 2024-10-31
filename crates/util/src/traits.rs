use bytemuck::{Pod, Zeroable};
use rend::{f32_le, f64_le, i32_le};

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct F32RetF32 {
    i: f32_le,
    o: f32_le,
}

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct F64RetF64 {
    i: f64_le,
    o: f64_le,
}

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct F32F32RetF32 {
    i: [f32_le; 2],
    o: f32_le,
}

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct F64F64RetF64 {
    i: [f64_le; 2],
    o: f64_le,
}

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct F32F32F32RetF32 {
    i: [f32_le; 3],
    o: f32_le,
}

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct F64F64F64RetF64 {
    i: [f64_le; 3],
    o: f64_le,
}

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct F32RetI32 {
    i: f32_le,
    o: i32_le,
}

#[derive(Clone, Copy, Zeroable, Pod)]
#[repr(C)]
pub struct F64RetI32 {
    i: f64_le,
    o: i32_le,
    _pad: u32,
}

pub trait PodRepr: Copy {
    type Output: Pod;
    fn to_pod(self) -> Self::Output;
    fn from_pod(v: Self::Output) -> Self;
}

impl PodRepr for ((f32,), f32) {
    type Output = F32RetF32;

    fn to_pod(self) -> Self::Output {
        F32RetF32 { i: self.0.0.into(), o: self.1.into() }
    }

    fn from_pod(v: Self::Output) -> Self {
        ((v.i.into(),), v.o.into())
    }
}

impl PodRepr for ((f64,), f64) {
    type Output = F64RetF64;

    fn to_pod(self) -> Self::Output {
        F64RetF64 { i: self.0.0.into(), o: self.1.into() }
    }

    fn from_pod(v: Self::Output) -> Self {
        ((v.i.into(),), v.o.into())
    }
}

impl PodRepr for ((f32, f32), f32) {
    type Output = F32F32RetF32;

    fn to_pod(self) -> Self::Output {
        F32F32RetF32 { i: [self.0.0.into(), self.0.1.into()], o: self.1.into() }
    }

    fn from_pod(v: Self::Output) -> Self {
        ((v.i[0].into(), v.i[1].into()), v.o.into())
    }
}

impl PodRepr for ((f64, f64), f64) {
    type Output = F64F64RetF64;

    fn to_pod(self) -> Self::Output {
        F64F64RetF64 { i: [self.0.0.into(), self.0.1.into()], o: self.1.into() }
    }

    fn from_pod(v: Self::Output) -> Self {
        ((v.i[0].into(), v.i[1].into()), v.o.into())
    }
}

impl PodRepr for ((f32, f32, f32), f32) {
    type Output = F32F32F32RetF32;

    fn to_pod(self) -> Self::Output {
        F32F32F32RetF32 { i: [self.0.0.into(), self.0.1.into(), self.0.2.into()], o: self.1.into() }
    }

    fn from_pod(v: Self::Output) -> Self {
        ((v.i[0].into(), v.i[1].into(), v.i[2].into()), v.o.into())
    }
}

impl PodRepr for ((f64, f64, f64), f64) {
    type Output = F64F64F64RetF64;

    fn to_pod(self) -> Self::Output {
        F64F64F64RetF64 { i: [self.0.0.into(), self.0.1.into(), self.0.2.into()], o: self.1.into() }
    }

    fn from_pod(v: Self::Output) -> Self {
        ((v.i[0].into(), v.i[1].into(), v.i[2].into()), v.o.into())
    }
}

impl PodRepr for ((f32,), i32) {
    type Output = F32RetI32;

    fn to_pod(self) -> Self::Output {
        F32RetI32 { i: self.0.0.into(), o: self.1.into() }
    }

    fn from_pod(v: Self::Output) -> Self {
        ((v.i.into(),), v.o.into())
    }
}

impl PodRepr for ((f64,), i32) {
    type Output = F64RetI32;

    fn to_pod(self) -> Self::Output {
        F64RetI32 { i: self.0.0.into(), o: self.1.into(), _pad: 0 }
    }

    fn from_pod(v: Self::Output) -> Self {
        ((v.i.into(),), v.o.into())
    }
}
