//! Test with "infinite precision"

#![cfg(feature = "multiprecision-tests")]
#![allow(unused)]

use std::ffi::c_int;
use std::sync::LazyLock;

use az::Az;
use libm_test::allowed_ulp;
use libm_test::gen::CachedInput;
use libm_test::rug_traits::MpFloat;
use libm_test::rug_traits::MpFloatThing;
use libm_test::rug_traits::ToSomething;
use libm_test::rug_traits::TupleAssign;
use libm_test::TRUE_DEFAULT_ULP;
use libm_test::{CheckOutput, GenerateInput, TupleCall};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rug::ops::CompleteRound;
use rug::ops::Pow;
use rug::Assign;

const SEED: [u8; 32] = *b"3.141592653589793238462643383279";

const NTESTS: usize = {
    let mut ntests = if cfg!(optimizations_enabled) {
        50000
    } else {
        500
    };

    // Tests can be pretty slow on non-64-bit targets and, for some reason, ppc.
    if !cfg!(target_pointer_width = "64") || cfg!(target_arch = "powerpc64") {
        ntests /= 5;
    }

    ntests
};

/// Tested inputs.
static TEST_CASES: LazyLock<CachedInput> = LazyLock::new(|| make_test_cases(NTESTS));

/// The first argument to `jn` and `jnf` is the number of iterations. Make this a reasonable
/// value so tests don't run forever.
static TEST_CASES_JN: LazyLock<CachedInput> = LazyLock::new(|| {
    // It is easy to overflow the stack with these in debug mode
    let iterations = if cfg!(optimizations_enabled) && cfg!(target_pointer_width = "64") {
        0xffff
    } else if cfg!(windows) {
        0x00ff
    } else {
        0x0fff
    };

    let mut cases = (&*TEST_CASES).clone();
    for case in cases.inputs_i32.iter_mut() {
        case.0 = iterations;
    }
    for case in cases.inputs_i32.iter_mut() {
        case.0 = iterations;
    }
    cases
});

fn make_test_cases(ntests: usize) -> CachedInput {
    let mut rng = ChaCha8Rng::from_seed(SEED);

    let inputs_i32 = (0..ntests).map(|_| rng.gen::<(i32, i32, i32)>()).collect();
    let inputs_f32 = (0..ntests).map(|_| rng.gen::<(f32, f32, f32)>()).collect();
    let inputs_f64 = (0..ntests).map(|_| rng.gen::<(f64, f64, f64)>()).collect();

    CachedInput {
        inputs_f32,
        inputs_f64,
        inputs_i32,
    }
}

macro_rules! musl_rand_tests {
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
        attrs: [$($meta:meta)*]
        fn_extra: $rug_expr:expr,
    ) => {
        paste::paste! {
            #[test]
            $(#[$meta])*
            fn [< multiprec_random_ $fn_name >]() {
                let fname = stringify!($fn_name);
                let inputs = if fname == "jn" || fname == "jnf" {
                    &TEST_CASES_JN
                } else {
                    &TEST_CASES
                };

                let ulp = allowed_ulp(fname, TRUE_DEFAULT_ULP);

                let cases = <CachedInput as GenerateInput<$RustArgs>>::get_cases(inputs);
                let mut mp_res = <$RustArgs>::new_mpfloat(128);
                let mut x = <$RugFn as MpFloatThing<f64>>::create(128);

                for input in cases {
                    // <$RugFn >::assign_values(input, &mut x);


                    input.set_values(&mut mp_res);
                    mp_res = mp_res.call($rug_expr as $RugFn);
                    let mp_res: $RustRet = mp_res.do_thing();

                    // let mres = input.call(musl::$fn_name as $CFn);
                    let cres = input.call(libm::$fn_name as $RustFn);

                    mp_res.validate(cres, input, ulp);
                }
            }
        }
    };
}

libm_macros::for_each_function! {
    callback: musl_rand_tests,
    attributes: [],
    skip: [

        acosf,acosh,acoshf,asin,asinf,asinh,asinhf,atan,atan2,atan2f,
        atanf,atanh,atanhf,cbrt,cbrtf,ceil,ceilf,copysign,copysignf,cos,cosf,
        cosh,coshf,erf,erff,exp,exp10,exp10f,exp2,exp2f,expf,expm1,expm1f,
        fabs,fabsf,fdim,fdimf,floor,floorf,fma,fmaf,fmax,fmaxf,
        fmin,fminf,fmod,fmodf,frexp,frexpf,hypot,hypotf,ilogb,ilogbf,j0,j0f,
        j1,j1f,jn,jnf,ldexp,ldexpf,lgamma,lgamma_r,lgammaf,lgammaf_r,log,log10,
        log10f,log1p,log1pf,log2,log2f,logf,modf,modff,nextafter,nextafterf,pow,powf,
        remainder,remainderf,remquo,remquof,rint,rintf,
        round,roundf,scalbn,scalbnf,sin,sincos,sincosf,sinf,sinh,sinhf,sqrt,sqrtf,
        tan,tanf,tanh,tanhf,tgamma,tgammaf,trunc,truncf,

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
        // (lgamma_r | lgammaf_r) => |x| {
        //     let f = x.ln_gamma();
        //     let i = f.cmp(0) as i32;
        //     (f, i)
        // },
        expm1 | expm1f => MpFloat::exp_m1,
        fabs | fabsf => MpFloat::abs,
        fdim | fdimf => MpFloat::positive_diff,
        fma | fmaf => |x: MpFloat, y: &MpFloat, z: &MpFloat| {
            let res = (&x * y) + z;
            res.complete(128)
        },
        fmax | fmaxf => MpFloat::max,
        fmin | fminf => MpFloat::min,
        fmod | fmodf => |x: MpFloat, y: &MpFloat| {
            x % y
        },
        lgamma | lgammaf => MpFloat::ln_gamma,
        log | logf => MpFloat::ln,
        log1p | log1pf => MpFloat::log10_1p,
        nextafter | nextafterf => |mut x: MpFloat, y: &MpFloat| {
            x.next_toward(y);
            x
        },
        pow | powf => |x: MpFloat, y: &MpFloat| x.pow(y),
        rint | rintf => |x: MpFloat| x.round(),
        tgamma | tgammaf => MpFloat::gamma,
        sincos | sincosf => |x: MpFloat| {
            let cos = MpFloat::new(128);
            x.sin_cos(cos)

        },

        // MpFloat::sin_cos,
        // (sincos | sincosf) => MpFloat::sin_cos,
        _ => MpFloat::MACRO_FN_NAME_NORMALIZED,
    }
}
