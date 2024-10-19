//! Test with "infinite precision"

#![cfg(feature = "multiprecision-tests")]
#![allow(unused)]

use std::ffi::c_int;
use std::sync::LazyLock;

use az::Az;
use libm_test::allowed_ulp;
use libm_test::gen::CachedInput;
use libm_test::rug_traits::ToSomething;
use libm_test::rug_traits::TupleAssign;
use libm_test::TRUE_DEFAULT_ULP;
use libm_test::{CheckOutput, GenerateInput, TupleCall};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
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
        attrs: [$($meta:meta)*]
        fn_extra: $rug_fn_name:expr,
    ) => {
        paste::paste! {
            #[test]
            // $(#[$meta])*
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

                for input in cases {
                    input.set_values(&mut mp_res);
                    mp_res = mp_res.call($rug_fn_name);

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
        expm1f,
        rintf,
        rint,
        logf,
        log,
        log1p,
        log1pf,
        expm1,
        erf,
        fdimf,
        // fmaxf,
        // fminf,
        fmodf,
        nextafterf,
        powf,
        fdim,
        // fmax,
        // fmin,
        fmod,
        nextafter,
        pow,
        fma,
        fmaf,
        ilogbf,
        ilogb,
        jnf,
        jn,
        scalbnf,
        ldexpf,
        scalbn,
        ldexp,
        modff,
        modf,
        frexpf,
        lgammaf_r,
        frexp,
        lgamma_r,
        remquof,
        remquo,
        sincosf,
        sincos,

    ],
    fn_extra: match MACRO_FN_NAME {
        (fabs | fabsf) => rug::Float::abs,
        (lgamma | lgammaf) => rug::Float::ln_gamma,
        (tgamma | tgammaf) => rug::Float::gamma,
        (fmin | fminf) => rug::Float::min,
        (fmax | fmaxf) => rug::Float::max,
        _ => rug::Float::MACRO_FN_NAME_NORMALIZED,
    }
}
