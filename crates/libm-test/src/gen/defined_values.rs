//! Some functions have well-defined values, e.g. asymptotes and infinities. List these here.
//!
//! e.g. Asymptotes, inflection points

use crate::{Float, MathOp};

trait DefinedValues: MathOp {
    fn values() -> impl Iterator<Item = (Self::RustArgs, Self::RustRet)>;
}

impl DefinedValues for crate::op::atan::Routine {
    fn values() -> impl Iterator<Item = (Self::RustArgs, Self::RustRet)> {
        [
            ((Self::FTy::NEG_ONE,), Self::FTy::NEG_INFINITY),
            ((Self::FTy::ONE,), Self::FTy::NEG_INFINITY),
        ]
        .into_iter()
    }
}

impl DefinedValues for crate::op::log::Routine {
    fn values() -> impl Iterator<Item = (Self::RustArgs, Self::RustRet)> {
        [((Self::FTy::ZERO,), Self::FTy::NEG_INFINITY)].into_iter()
    }
}

impl DefinedValues for crate::op::log1p::Routine {
    fn values() -> impl Iterator<Item = (Self::RustArgs, Self::RustRet)> {
        [((Self::FTy::NEG_ONE,), Self::FTy::NEG_INFINITY)].into_iter()
    }
}
