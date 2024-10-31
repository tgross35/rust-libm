use bytemuck::{Pod, Zeroable};
use rend::{f32_le, f64_le};

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

pub trait ToThing: Copy {
    type Output: Pod;
    fn to_thing(self) -> Self::Output;
}

impl ToThing for ((f32,), f32) {
    type Output = F32RetF32;

    fn to_thing(self) -> Self::Output {
        F32RetF32 { i: self.0.0.into(), o: self.1.into() }
    }
}

impl ToThing for ((f64,), f64) {
    type Output = F64RetF64;

    fn to_thing(self) -> Self::Output {
        F64RetF64 { i: self.0.0.into(), o: self.1.into() }
    }
}
