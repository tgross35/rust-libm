//! Test with "infinite precision"
#![allow(unused)]
use std::ffi::c_int;
use std::sync::LazyLock;

use az::Az;
use libm_test::gen::CachedInput;
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

/// ULP allowed to differ from musl (note that musl itself may not be accurate).
const ALLOWED_ULP: u32 = 1;

/// Certain functions have different allowed ULP (consider these xfail).
///
/// Currently this includes:
/// - gamma functions that have higher errors
/// - 32-bit functions fall back to a less precise algorithm.
const ULP_OVERRIDES: &[(&str, u32)] = &[
    // #[cfg(x86_no_sse)]
    // ("asinhf", 6),
    // ("lgamma", 6),
    // ("lgamma_r", 6),
    // ("lgammaf", 6),
    // ("lgammaf_r", 6),
    // ("tanh", 4),
    // ("tgamma", 8),
    // #[cfg(not(target_pointer_width = "64"))]
    // ("exp10", 4),
    // #[cfg(not(target_pointer_width = "64"))]
    // ("exp10f", 4),
];

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

// macro_rules! infinite_tests {
//     (@each_signature
//         SysArgsTupleTy: $_sys_argty:ty,
//         RustArgsTupleTy: $argty:ty,
//         SysFnTy: $fnty_sys:ty,
//         RustFnTy: $fnty_rust:ty,
//         functions: [$( {
//             attrs: [$($fn_meta:meta),*],
//             fn_name: $name:ident,
//         } ),*],
//     ) => { paste::paste! {
//         $(
//             #[test]
//             $(#[$fn_meta])*
//             fn [< musl_random_ $name >]() {
//                 let fname = stringify!($name);
//                 let inputs = if fname == "jn" || fname == "jnf" {
//                     &TEST_CASES_JN
//                 } else {
//                     &TEST_CASES
//                 };

//                 let ulp = match ULP_OVERRIDES.iter().find(|(name, _val)| name == &fname) {
//                     Some((_name, val)) => *val,
//                     None => ALLOWED_ULP,
//                 };

//                 let cases = <CachedInput as GenerateInput<$argty>>::get_cases(inputs);
//                 for input in cases {
//                     let mres = input.call(musl::$name as $fnty_sys);
//                     let cres = input.call(libm::$name as $fnty_rust);

//                     mres.validate(cres, input, ulp);
//                 }
//             }
//         )*
//     } };

//     (@all_items$($tt:tt)*) => {};
// }

// libm::for_each_function!(infinite_tests);

#[test]
fn foobar_arcsin() {
    let fname = stringify!(asin);
    let inputs = if fname == "jn" || fname == "jnf" {
        &TEST_CASES_JN
    } else {
        &TEST_CASES
    };

    let ulp = match ULP_OVERRIDES.iter().find(|(name, _val)| name == &fname) {
        Some((_name, val)) => *val,
        None => ALLOWED_ULP,
    };

    let cases = <CachedInput as GenerateInput<(f64,)>>::get_cases(inputs);
    let mut f = rug::Float::new(128);

    for input in cases {
        f.assign(input.0);
        f = f.asin();
        let rres = f.to_f64();

        // let mres = input.call(musl::asin as fn(f64) -> f64);
        let cres = input.call(libm::asin as fn(f64) -> f64);

        rres.validate(cres, input, ulp);
    }
}
