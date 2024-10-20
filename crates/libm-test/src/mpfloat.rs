use az::Az;
// use rug::ops::Pow;
use rug::ops::PowAssign;
use rug::Assign;
pub use rug::Float as MpFloat;

// TODO
const PREC_F64: u32 = 1000;

pub trait MpOp {
    type Input;
    type Output;
    fn new() -> Self;
    fn assign_run(&mut self, input: Self::Input) -> Self::Output;
}

macro_rules! impl_mp_op {
    // Matcher for unary functions
    (
        fn_name: $fn_name:ident,
        CFn: $CFn:ty,
        CArgs: $CArgs:ty,
        CRet: $CRet:ty,
        RustFn: fn($_arg:ty,) -> $_ret:ty,
        RustArgs: $RustArgs:ty,
        RustRet: $RustRet:ty,
        fn_extra: $fn_name_normalized:expr,
    ) => {
        paste::paste! {
            pub mod $fn_name {
                use super::*;
                pub struct Operation(MpFloat);

                impl MpOp for Operation {
                    type Input = $RustArgs;
                    type Output = $RustRet;

                    fn new() -> Self {
                        // TODO precision
                        Self(MpFloat::new(PREC_F64))
                    }

                    fn assign_run(&mut self, input: Self::Input) -> Self::Output {
                        self.0.assign(input.0);
                        self.0.[< $fn_name_normalized _mut >]();
                        // TODO subnormalize
                        (&self.0).az::<Self::Output>()
                    }
                }
            }
        }
    };
    // Matcher for binary functions
    (
        fn_name: $fn_name:ident,
        CFn: $CFn:ty,
        CArgs: $CArgs:ty,
        CRet: $CRet:ty,
        RustFn: fn($_arg1:ty, $_arg2:ty,) -> $_ret:ty,
        RustArgs: $RustArgs:ty,
        RustRet: $RustRet:ty,
        fn_extra: $fn_name_normalized:expr,
    ) => {
        paste::paste! {
            pub mod $fn_name {
                use super::*;
                pub struct Operation(MpFloat, MpFloat);

                impl MpOp for Operation {
                    type Input = $RustArgs;
                    type Output = $RustRet;

                    fn new() -> Self {
                        // TODO precision
                        Self(MpFloat::new(PREC_F64), MpFloat::new(PREC_F64))
                    }

                    fn assign_run(&mut self, input: Self::Input) -> Self::Output {
                        self.0.assign(input.0);
                        self.1.assign(input.1);
                        self.0.[< $fn_name_normalized _mut >](&self.1);
                        // TODO subnormalize
                        (&self.0).az::<Self::Output>()
                    }
                }
            }
        }
    };
    // Matcher for ternary functions
    (
        fn_name: $fn_name:ident,
        CFn: $CFn:ty,
        CArgs: $CArgs:ty,
        CRet: $CRet:ty,
        RustFn: fn($_arg1:ty, $_arg2:ty, $_arg3:ty,) -> $_ret:ty,
        RustArgs: $RustArgs:ty,
        RustRet: $RustRet:ty,
        fn_extra: $fn_name_normalized:expr,
    ) => {
        paste::paste! {
            pub mod $fn_name {
                use super::*;
                pub struct Operation(MpFloat, MpFloat, MpFloat);

                impl MpOp for Operation {
                    type Input = $RustArgs;
                    type Output = $RustRet;

                    fn new() -> Self {
                        // TODO precision
                        Self(MpFloat::new(PREC_F64), MpFloat::new(PREC_F64), MpFloat::new(PREC_F64))
                    }

                    fn assign_run(&mut self, input: Self::Input) -> Self::Output {
                        self.0.assign(input.0);
                        self.1.assign(input.1);
                        self.2.assign(input.2);
                        self.0.[< $fn_name_normalized _mut >](&self.1, &self.2);
                        // TODO subnormalize
                        (&self.0).az::<Self::Output>()
                    }
                }
            }
        }
    };
}

libm_macros::for_each_function! {
    callback: impl_mp_op,
    skip: [
        fmod,fmodf,
        ilogb,ilogbf,
        nextafter,nextafterf,
        pow,powf,

        frexp,
        frexpf,
        ldexp,
        ldexpf,
        scalbn,
        scalbnf,

        jn,jnf,

        ilogb,
        ilogbf,
        lgamma_r,
        lgammaf_r,
        modf,
        modff,
        remquo,
        remquof,
        sincos,
        sincosf,
    ],
    fn_extra: match MACRO_FN_NAME {
        expm1 | expm1f => exp_m1,
        fabs | fabsf => abs,
        fdim | fdimf => positive_diff,
        fma | fmaf => mul_add,
        fmax | fmaxf => max,
        fmin | fminf => min,
        lgamma | lgammaf => ln_gamma,
        log | logf => ln,
        log1p | log1pf => log10_1p,
        rint | rintf => round,
        tgamma | tgammaf => gamma,
        _ => MACRO_FN_NAME_NORMALIZED
    }
}

/// Some functions are difficult to do in a generic way. Operationement them here.
macro_rules! impl_for_both {
    // Matcher for unary functions
    (
        $fty:ty, $suffix:literal
    ) => {
        paste::paste! {
            pub mod [<nextafter $suffix>] {
                use super::*;
                pub struct Operation(MpFloat, MpFloat);

                impl MpOp for Operation {
                    type Input = ($fty, $fty);
                    type Output = $fty;

                    fn new() -> Self {
                        // TODO precision
                        Self(MpFloat::new(PREC_F64), MpFloat::new(PREC_F64))
                    }

                    fn assign_run(&mut self, input: Self::Input) -> Self::Output {
                        self.0.assign(input.0);
                        self.1.assign(input.1);
                        self.0.next_toward(&self.1);

                        // TODO subnormalize
                        (&self.0).az::<$fty>()
                    }
                }
            }

            pub mod [<pow $suffix>] {
                use super::*;
                pub struct Operation(MpFloat, MpFloat);

                impl MpOp for Operation {
                    type Input = ($fty, $fty);
                    type Output = $fty;

                    fn new() -> Self {
                        // TODO precision
                        Self(MpFloat::new(PREC_F64), MpFloat::new(PREC_F64))
                    }

                    fn assign_run(&mut self, input: Self::Input) -> Self::Output {
                        self.0.assign(input.0);
                        self.1.assign(input.1);
                        self.0.pow_assign(&self.1);

                        // TODO subnormalize
                        (&self.0).az::<$fty>()
                    }
                }
            }

            pub mod [<sincos $suffix>] {
                use super::*;
                pub struct Operation(MpFloat, MpFloat);

                impl MpOp for Operation {
                    type Input = ($fty,);
                    type Output = ($fty, $fty);

                    fn new() -> Self {
                        // TODO precision
                        Self(MpFloat::new(PREC_F64), MpFloat::new(PREC_F64))
                    }

                    fn assign_run(&mut self, input: Self::Input) -> Self::Output {
                        self.0.assign(input.0);
                        self.1.assign(0.0);
                        self.0.sin_cos_mut(&mut self.1);

                        // TODO subnormalize
                        ((&self.0).az::<$fty>(), (&self.1).az::<$fty>())
                    }
                }
            }
        }
    };
}

impl_for_both!(f32, "f");
impl_for_both!(f64, "");
