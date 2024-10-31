use std::collections::BTreeMap;
use std::fs;
use std::io::{BufWriter, Write};
use std::path::Path;

use bytemuck::{NoUninit, bytes_of};
use libm_test::func::MathOp;
use libm_test::gen::{CachedInput, random};
use libm_test::mpfloat::MpOp;
use libm_test::{CheckBasis, CheckCtx, GenerateInput, multiprec_allowed_ulp};

use crate::traits::ToThing;
// use rkyv::collections::btree_map::ArchivedBTreeMap;
// use rkyv::rancor::Error;
// use rkyv::{Archive, Deserialize, Serialize, deserialize};

// #[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
// enum Foo {
//     F32RetF32(Vec<((f32,), f32)>),
//     F64RetF64(Vec<((f64,), f64)>),
// }

fn test_one<Func, W>(mut w: W)
where
    W: Write,
    Func: MathOp + MpOp,
    (Func::RustArgs, Func::RustRet): ToThing,
    // <Func as MathOp>::RustArgs: NoUninit,
    // <Func as MathOp>::RustRet: NoUninit,
    CachedInput: GenerateInput<Func::RustArgs>,
{
    let name = Func::NAME_STR;

    let ulp = multiprec_allowed_ulp(name);
    let mut mp_vals = Func::new_mp();
    let ctx = CheckCtx::new(ulp, name, CheckBasis::Mpfr);
    let cases = random::get_test_cases::<Func::RustArgs>(&ctx);

    for input in cases {
        let mp_res = Func::run(&mut mp_vals, input);
        let x = (input, mp_res).to_thing();
        w.write(bytes_of(&x)).unwrap();
    }
}

pub fn do_thing(path: &Path) {
    let p = path.with_file_name("sinf.bin");
    let f = fs::File::create(p).unwrap();
    let mut stream = BufWriter::new(f);
    test_one::<libm_test::func::sinf::Func, _>(&mut stream);
    stream.flush().unwrap();

    let p = path.with_file_name("sin.bin");
    let f = fs::File::create(p).unwrap();
    let mut stream = BufWriter::new(f);
    test_one::<libm_test::func::sin::Func, _>(&mut stream);
    stream.flush().unwrap();

    let p = path.with_file_name("cosf.bin");
    let f = fs::File::create(p).unwrap();
    let mut stream = BufWriter::new(f);
    test_one::<libm_test::func::cosf::Func, _>(&mut stream);
    stream.flush().unwrap();

    let p = path.with_file_name("cos.bin");
    let f = fs::File::create(p).unwrap();
    let mut stream = BufWriter::new(f);
    test_one::<libm_test::func::cos::Func, _>(&mut stream);
    stream.flush().unwrap();
}
