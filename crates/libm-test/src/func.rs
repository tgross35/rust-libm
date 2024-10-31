//! Types representing individual functions.
//!
//! Each function gets a module with its function name, e.g. `mod sinf { /* ... */ }`. The
//! module contains a unit struct `Func` which implements `MathOp`.

use crate::{CheckOutput, Float, TupleCall};

/// An enum representing each possible function name.
#[libm_macros::function_enum(BaseName)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Name {}

/// The name without any type specifier, e.g. `sin` and `sinf` both become `sin`.
#[libm_macros::base_name_enum]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BaseName {}

/// Attributes ascribed to a `libm` function including signature, type information,
/// and naming.
pub trait MathOp {
    /// The float type used for this operation.
    type FTy: Float;

    /// The function type representing the signature in a C library.
    type CFn: Copy;

    /// Arguments passed to the C library function as a tuple. These may include `&mut` return
    /// values.
    type CArgs<'a>
    where
        Self: 'a;

    /// The type returned by C implementations.
    type CRet;

    /// The signature of the Rust function as a `fn(...) -> ...` type.
    type RustFn: Copy;

    /// Arguments passed to the Rust library function as a tuple.
    ///
    /// The required `TupleCall` bounds ensure this type can be passed either to the C function or
    /// to the Rust function.
    type RustArgs: Copy
        + TupleCall<Self::RustFn, Output = Self::RustRet>
        + TupleCall<Self::CFn, Output = Self::RustRet>;

    /// Type returned from the Rust function.
    type RustRet: CheckOutput<Self::RustArgs>;

    /// The name of this function, including suffix (e.g. `sin`, `sinf`).
    const NAME: Name;

    /// The name as a string.
    const NAME_STR: &'static str = Self::NAME.as_str();

    /// The name of the function excluding the type suffix, e.g. `sin` and `sinf` are both `sin`.
    const BASE_NAME: BaseName = Self::NAME.base_name();

    /// The function in `libm` which can be called.
    const LIBM_FN: Self::RustFn;
}

macro_rules! do_thing {
    // Matcher for unary functions
    (
        fn_name: $fn_name:ident,
        FTy: $FTy:ty,
        CFn: $CFn:ty,
        CArgs: $CArgs:ty,
        CRet: $CRet:ty,
        RustFn: $RustFn:ty,
        RustArgs: $RustArgs:ty,
        RustRet: $RustRet:ty,
    ) => {
        paste::paste! {
            pub mod $fn_name {
                use super::*;
                pub struct Func;

                impl MathOp for Func {
                    type FTy = $FTy;
                    type CFn = for<'a> $CFn;
                    type CArgs<'a> = $CArgs where Self: 'a;
                    type CRet = $CRet;
                    type RustFn = $RustFn;
                    type RustArgs = $RustArgs;
                    type RustRet = $RustRet;

                    const NAME: Name = Name::[< $fn_name:camel >];
                    const LIBM_FN: Self::RustFn = libm::$fn_name;
                }
            }

        }
    };
}

libm_macros::for_each_function! {
    callback: do_thing,
    emit_types: all,
}
