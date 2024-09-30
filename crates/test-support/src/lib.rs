#![allow(unused)] // TODO remove

use core::str::FromStr;
use core::{fmt, ops};
pub use rug::Float as MpF;
use std::collections::BTreeMap;
use std::marker::PhantomData;

#[macro_export]
macro_rules! float_test {
    () => {
        mod auto_tests {
            #[test]
            fn foo() {
                assert!(true);
            }
        }
    };
}

pub fn run_test_binop<F, G>(op: impl Fn(F) -> F, mp_op: impl Fn(MpF) -> MpF, ulp: u32)
where
    F: Float,
    G: Generator<F>,
    MpF: rug::Assign<F>,
{
    let mut gen = G::new();
    for val in gen {
        let actual = op(val);
        let expected = mp_op(MpF::with_val(F::MP_PRECISION, val));
    }
}

/// Integer types.
pub trait Int:
    Clone
    + Copy
    + fmt::Debug
    + fmt::Display
    + fmt::LowerHex
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Shl<u32, Output = Self>
    + ops::Shr<u32, Output = Self>
    + ops::BitAnd<Output = Self>
    + ops::BitOr<Output = Self>
    + ops::Not<Output = Self>
    + ops::AddAssign
    + ops::BitAndAssign
    + ops::BitOrAssign
    + From<u8>
    + TryFrom<i8>
    + TryFrom<u32, Error: fmt::Debug>
    + TryFrom<u64, Error: fmt::Debug>
    + TryFrom<u128, Error: fmt::Debug>
    + TryInto<u64, Error: fmt::Debug>
    + TryInto<u32, Error: fmt::Debug>
    + PartialOrd
{
    type Signed: Int;
    type Bytes: Default + AsMut<[u8]>;

    const BITS: u32;
    const ZERO: Self;
    const ONE: Self;
    const MAX: Self;

    fn to_signed(self) -> Self::Signed;
    fn wrapping_neg(self) -> Self;
    fn trailing_zeros(self) -> u32;

    fn hex(self) -> String {
        format!("{self:x}")
    }
}

macro_rules! impl_int {
    ($($uty:ty, $sty:ty);+) => {
        $(
            impl Int for $uty {
                type Signed = $sty;
                type Bytes = [u8; Self::BITS as usize / 8];
                const BITS: u32 = Self::BITS;
                const ZERO: Self = 0;
                const ONE: Self = 1;
                const MAX: Self = Self::MAX;
                fn to_signed(self) -> Self::Signed {
                    self.try_into().unwrap()
                }
                fn wrapping_neg(self) -> Self {
                    self.wrapping_neg()
                }
                fn trailing_zeros(self) -> u32 {
                    self.trailing_zeros()
                }
            }

            impl Int for $sty {
                type Signed = Self;
                type Bytes = [u8; Self::BITS as usize / 8];
                const BITS: u32 = Self::BITS;
                const ZERO: Self = 0;
                const ONE: Self = 1;
                const MAX: Self = Self::MAX;
                fn to_signed(self) -> Self::Signed {
                    self
                }
                fn wrapping_neg(self) -> Self {
                    self.wrapping_neg()
                }
                fn trailing_zeros(self) -> u32 {
                    self.trailing_zeros()
                }
            }
        )+
    }
}

impl_int!(u32, i32; u64, i64);

/// Floating point types.
pub trait Float:
    Copy + fmt::Debug + fmt::LowerExp + FromStr<Err: fmt::Display> + Sized + Send + 'static
{
    /// Unsigned integer of same width
    type Int: Int<Signed = Self::SInt>;
    type SInt: Int;

    /// Total bits
    const BITS: u32;

    /// (Stored) bits in the mantissa)
    const MAN_BITS: u32;

    /// Bits in the exponent
    const EXP_BITS: u32 = Self::BITS - Self::MAN_BITS - 1;

    /// A saturated exponent (all ones)
    const EXP_SAT: u32 = (1 << Self::EXP_BITS) - 1;

    /// The exponent bias, also its maximum value
    const EXP_BIAS: u32 = Self::EXP_SAT >> 1;

    const MAN_MASK: Self::Int;
    const SIGN_MASK: Self::Int;
    const MP_PRECISION: u32 = Self::BITS * 3 / 2;

    fn from_bits(i: Self::Int) -> Self;
    fn to_bits(self) -> Self::Int;

    /// Rational constants associated with this float type.
    fn constants() -> &'static Constants;

    fn is_sign_negative(self) -> bool {
        (self.to_bits() & Self::SIGN_MASK) > Self::Int::ZERO
    }

    /// Exponent without adjustment for bias.
    fn exponent(self) -> u32 {
        ((self.to_bits() >> Self::MAN_BITS) & Self::EXP_SAT.try_into().unwrap())
            .try_into()
            .unwrap()
    }

    fn mantissa(self) -> Self::Int {
        self.to_bits() & Self::MAN_MASK
    }
}

macro_rules! impl_float {
    ($($fty:ty, $ity:ty, $bits:literal);+) => {
        $(
            impl Float for $fty {
                type Int = $ity;
                type SInt = <Self::Int as Int>::Signed;
                const BITS: u32 = $bits;
                const MAN_BITS: u32 = Self::MANTISSA_DIGITS - 1;
                const MAN_MASK: Self::Int = (Self::Int::ONE << Self::MAN_BITS) - Self::Int::ONE;
                const SIGN_MASK: Self::Int = Self::Int::ONE << (Self::BITS-1);
                fn from_bits(i: Self::Int) -> Self { Self::from_bits(i) }
                fn to_bits(self) -> Self::Int { self.to_bits() }
                fn constants() -> &'static Constants {
                    use std::sync::LazyLock;
                    static CONSTANTS: LazyLock<Constants> = LazyLock::new(Constants::new::<$fty>);
                    &CONSTANTS
                }
            }
        )+
    }
}

impl_float!(f32, u32, 32; f64, u64, 64);

/// Rational property-related constants for a specific float type.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Constants {
    /// The minimum positive value (a subnormal).
    min_subnormal: MpF,
    /// The maximum possible finite value.
    max: MpF,
    /// Cutoff between rounding to zero and rounding to the minimum value (min subnormal).
    zero_cutoff: MpF,
    /// Cutoff between rounding to the max value and rounding to infinity.
    inf_cutoff: MpF,
    /// Opposite of `inf_cutoff`
    neg_inf_cutoff: MpF,
    /// The powers of two for all relevant integers.
    powers_of_two: BTreeMap<i32, MpF>,
    /// Half of each power of two. ULP = "unit in last position".
    ///
    /// This is a mapping from integers to half the precision available at that exponent. In other
    /// words, `0.5 * 2^n` = `2^(n-1)`, which is half the distance between `m * 2^n` and
    /// `(m + 1) * 2^n`, m ∈ ℤ.
    ///
    /// So, this is the maximum error from a real number to its floating point representation,
    /// assuming the float type can represent the exponent.
    half_ulp: BTreeMap<i32, MpF>,
    // Handy to have around so we don't need to reallocate for it
    // two: BigInt,
}

impl Constants {
    pub fn new<F: Float>() -> Self {
        todo!()
        // let two_int = &BigInt::from_u32(2).unwrap();
        // let two = &MpF::from_integer(2.into());

        // // The minimum subnormal (aka minimum positive) value. Most negative power of two is the
        // // minimum exponent (bias - 1) plus the extra from shifting within the mantissa bits.
        // let min_subnormal = two.pow(-(F::EXP_BIAS + F::MAN_BITS - 1).to_signed());

        // // The maximum value is the maximum exponent with a fully saturated mantissa. This
        // // is easiest to calculate by evaluating what the next value up would be if representable
        // // (zeroed mantissa, exponent increments by one, i.e. `2^(bias + 1)`), and subtracting
        // // a single LSB (`2 ^ (-mantissa_bits)`).
        // let max = (two - two.pow(-F::MAN_BITS.to_signed())) * (two.pow(F::EXP_BIAS.to_signed()));
        // let zero_cutoff = &min_subnormal / two_int;

        // let inf_cutoff = &max + two_int.pow(F::EXP_BIAS - F::MAN_BITS - 1);
        // let neg_inf_cutoff = -&inf_cutoff;

        // let powers_of_two: BTreeMap<i32, _> =
        //     (POWERS_OF_TWO_RANGE).map(|n| (n, two.pow(n))).collect();
        // let mut half_ulp = powers_of_two.clone();
        // half_ulp.iter_mut().for_each(|(_k, v)| *v = &*v / two_int);

        // Self {
        //     min_subnormal,
        //     max,
        //     zero_cutoff,
        //     inf_cutoff,
        //     neg_inf_cutoff,
        //     powers_of_two,
        //     half_ulp,
        //     two: two_int.clone(),
        // }
    }
}

/// A test generator. Should provide an iterator that produces unique patterns to parse.
pub trait Generator<F: Float>: Iterator<Item = F> {
    /// Constructor for this test generator.
    fn new() -> Self;
}

struct Basic<F: Float> {
    _a: PhantomData<F>,
}

impl<F: Float> Iterator for Basic<F> {
    type Item = F;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<F: Float> Generator<F> for Basic<F> {
    fn new() -> Self {
        todo!()
    }
}
