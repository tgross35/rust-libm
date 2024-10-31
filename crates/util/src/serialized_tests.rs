use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;

use bytemuck::bytes_of;
use libm_macros::for_each_function;
use libm_test::func::MathOp;
use libm_test::gen::{CachedInput, random};
use libm_test::mpfloat::MpOp;
use libm_test::{CheckBasis, CheckCtx, GenerateInput, multiprec_allowed_ulp};

use crate::traits::PodRepr;

macro_rules! to_file {
    (
        fn_name: $fn_name:ident,
        extra: [$path:ident, $pool:ident],
    ) => {
        test_one::<libm_test::func::$fn_name::Func>($path, &$pool);
    };
}

fn test_one<Func>(path: &Path, scope: &rayon::Scope)
where
    Func: MathOp + MpOp,
    (Func::RustArgs, Func::RustRet): PodRepr,
    // <Func as MathOp>::RustArgs: NoUninit,
    // <Func as MathOp>::RustRet: NoUninit,
    CachedInput: GenerateInput<Func::RustArgs>,
{
    let name = Func::NAME_STR;
    let path = path.join(name).with_extension("bin");

    scope.spawn(move |_| {
        println!("Started generating {}", path.display());

        let f = fs::File::create(&path).unwrap();
        let mut w = BufWriter::new(f);

        let ulp = multiprec_allowed_ulp(name);
        let mut mp_vals = Func::new_mp();
        let ctx = CheckCtx::new(ulp, name, CheckBasis::Mpfr);
        let cases = random::get_test_cases::<Func::RustArgs>(&ctx);

        for input in cases {
            let mp_res = Func::run(&mut mp_vals, input);
            let x = (input, mp_res).to_pod();
            w.write(bytes_of(&x)).unwrap();
        }

        w.flush().unwrap();
        println!("Finished generating {}", path.display());
    });
}

pub fn create(path: &Path) {
    let pool = rayon::ThreadPoolBuilder::new().build().unwrap();

    pool.scope(|scope| {
        for_each_function! {
             callback: to_file,
             extra: [path, scope],
             skip: [
                // wip
                j0,
                j0f,
                j1,
                j1f,
                jn,
                jnf,
                modf,
                modff,
                lgammaf_r,
                lgamma_r,
                sincos,
                sincosf,

                // No MPFR test
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
                nextafter,
                nextafterf,
             ]
        }
    });
}
