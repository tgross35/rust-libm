//! Test with "infinite precision"

#![cfg(feature = "test-multiprecision")]

use libm_test::func::MathOp;
use libm_test::gen::{CachedInput, random};
use libm_test::mpfloat::MpOp;
use libm_test::{
    CheckBasis, CheckCtx, CheckOutput, GenerateInput, TupleCall, multiprec_allowed_ulp,
};

/// Implement a test against MPFR with random inputs.
macro_rules! multiprec_rand_tests {
    (
        fn_name: $fn_name:ident,
        attrs: [$($meta:meta)*]
    ) => {
        paste::paste! {
            #[test]
            $(#[$meta])*
            fn [< multiprec_random_ $fn_name >]() {
                test_one::<libm_test::func::$fn_name::Func>();
            }
        }
    };
}

fn test_one<Func>()
where
    Func: MathOp + MpOp,
    CachedInput: GenerateInput<Func::RustArgs>,
{
    let name = Func::NAME_STR;

    let ulp = multiprec_allowed_ulp(name);
    let mut mp_vals = Func::new_mp();
    let ctx = CheckCtx::new(ulp, name, CheckBasis::Mpfr);
    let cases = random::get_test_cases::<Func::RustArgs>(&ctx);

    for input in cases {
        let mp_res = Func::run(&mut mp_vals, input);
        let crate_res = input.call(Func::LIBM_FN);

        crate_res.validate(mp_res, input, &ctx).unwrap();
    }
}

libm_macros::for_each_function! {
    callback: multiprec_rand_tests,
    attributes: [
        // Also an assertion failure on i686: at `MPFR_ASSERTN (! mpfr_erangeflag_p ())`
        #[ignore = "large values are infeasible in MPFR"]
        [jn, jnf],
    ],
    skip: [
        // FIXME: MPFR tests needed
        frexp,
        frexpf,
        ilogb,
        ilogbf,
        ldexp,
        ldexpf,
        modf,
        modff,
        remquo,
        remquof,
        scalbn,
        scalbnf,

        // FIXME: test needed, see
        // https://github.com/rust-lang/libm/pull/311#discussion_r1818273392
        nextafter,
        nextafterf,
    ],
}
