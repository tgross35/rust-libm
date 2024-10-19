use az::Az;
// use rug::ops::Pow;
use rug::Assign;
pub use rug::Float as MpFloat;

// FIXME
const PREC_F64: u32 = 1000;

pub trait MpOp {
    type Input;
    type Output;
    fn new() -> Self;
    fn assign_run(&mut self, input: Self::Input) -> Self::Output;
}

macro_rules! tbd {
    (unary $inty:ty, $outty:ty, $convert:ident: [$($fn_name:ident,)* ]) => { $(
        paste::paste! {
            pub struct [< $fn_name:camel >](MpFloat);

            impl MpOp for [< $fn_name:camel >] {
                type Input = (f64,);
                type Output = f64;

                fn new() -> Self {
                    Self(MpFloat::new(PREC_F64))
                }

                fn assign_run(&mut self, input: Self::Input) -> Self::Output {
                    self.0.assign(input.0);
                    self.0.acos_mut();
                    self.0.to_f64()
                }
            }
        })*
    };
}

macro_rules! tbd2 {
    (
        fn_name: $fn_name:ident,
        CFn: $CFn:ty,
        CArgs: $CArgs:ty,
        CRet: $CRet:ty,
        RustFn: $RustFn:ty,
        RustArgs: $RustArgs:ty,
        RustRet: $RustRet:ty,
        RugFn: $RugFn:ty,
        RugArgs: $RugArgs:ty,
        RugRet: $RugRet:ty,
        fn_extra: $fn_name_normalized:expr,
    ) => {
        paste::paste! {
            pub struct [< $fn_name:camel >](MpFloat);

            impl MpOp for [< $fn_name:camel >] {
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
    };
}

libm_macros::for_each_function! {
    callback: tbd2,
    skip: [
        atan2,atan2f,
        copysign,copysignf,
        fdim,fdimf,
        fma,fmaf,
        fmax,fmaxf,
        fmin,fminf,
        fmod,fmodf,frexp,frexpf,
        hypot,hypotf,
        ilogb,ilogbf,
        // j1,j1f,
        jn,jnf,
        ldexp,ldexpf,
        lgamma,lgamma_r,lgammaf,lgammaf_r,
        modf,modff,
        nextafter,nextafterf,
        pow,powf,
        remainder,remainderf,
        remquo,remquof,
        rint,rintf,
        round,roundf,
        scalbn,scalbnf,
        sincos,sincosf,
        tgamma,tgammaf,
        trunc,truncf,

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
        // sincos,
        sincosf,
    ],
    fn_extra: match MACRO_FN_NAME {
        expm1 | expm1f => exp_m1,
        fabs | fabsf => abs,
        log | logf => ln,
        log1p | log1pf => log10_1p,
        _ => MACRO_FN_NAME_NORMALIZED
    }
}
