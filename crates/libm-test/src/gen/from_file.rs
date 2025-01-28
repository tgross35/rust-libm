//! Parse a file with one input per line in hex float syntax.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::path::Path;

use libm::support::Float;

use crate::FloatExt;

pub fn get_test_cases<F: FloatExt>(fname: impl AsRef<Path>) -> impl Iterator<Item = F> {
    let fname = fname.as_ref();

    let f = File::open(fname).unwrap_or_else(|e| panic!("unable to open `{fname:?}`: {e}"));

    // Files can be multiple MB so optimize reading.
    let mut reader = BufReader::new(f);
    let mut buf = String::new();

    iter::from_fn(move || {
        let s = loop {
            buf.clear();
            let len = reader.read_line(&mut buf).unwrap();
            if len == 0 {
                // End of the file
                return None;
            }

            // Truncate everything after a `#` on the line (remove comments)
            buf.truncate(buf.split_once("#").map(|v| v.0).unwrap_or(&buf).len());
            let trimmed = buf.trim_start();

            // If this is a nonempty string (not a comment), return
            if !trimmed.is_empty() {
                break trimmed;
            }
        };

        let ret = match s {
            "+snan" => consts.min_snan,
            "-snan" => consts.neg_min_snan,
            "+qnan" => consts.pos_nan,
            "-qnnn" => consts.neg_nan,
            "+inf" => F::INFINITY,
            "-inf" => F::NEG_INFINITY,
            _ => F::from_hex_str(s),
        };

        Some(ret)
    })
}

trait Foo {
    fn foo(input: &str) -> Self;
}

macro_rules! impl_parse_tuple {
    ($ty:ty) => {
        impl Foo for ($ty,) {
            fn foo(input: &str) -> Self {
                (parse(input, 0),)
            }
        }

        impl Foo for ($ty, $ty) {
            fn foo(input: &str) -> Self {
                (parse(input, 0), parse(input, 1))
            }
        }

        impl Foo for ($ty, i32) {
            fn foo(input: &str) -> Self {
                (parse(input, 0), parse(input, 1))
            }
        }

        impl Foo for (i32, $ty) {
            fn foo(input: &str) -> Self {
                (parse(input, 0), parse(input, 1))
            }
        }

        impl Foo for ($ty, $ty, $ty) {
            fn foo(input: &str) -> Self {
                (parse(input, 0), parse(input, 1), parse(input, 2))
            }
        }
    };
}

impl_parse_tuple!(f32);
impl_parse_tuple!(f64);

#[cfg(f16_enabled)]
impl_parse_tuple!(f16);
#[cfg(f128_enabled)]
impl_parse_tuple!(f128);

fn parse_float<F: FloatExt>(s: &str) -> F {
    let consts = F::consts();

    let ret = match s {
        "+snan" => consts.min_snan,
        "-snan" => consts.neg_min_snan,
        "+qnan" => consts.pos_nan,
        "-qnnn" => consts.neg_nan,
        "+inf" => F::INFINITY,
        "-inf" => F::NEG_INFINITY,
        _ => F::from_hex_str(s),
    };
    ret
}
