use core::ops;

/// Wrapper to extract the integer type half of the float's size
pub(crate) type HalfRep<F> = <<F as Float>::Int as DInt>::H;

/// Trait for some basic operations on floats
#[allow(dead_code)]
pub(crate) trait Float:
    Copy
    + core::fmt::Debug
    + PartialEq
    + PartialOrd
    + ops::AddAssign
    + ops::MulAssign
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Div<Output = Self>
    + ops::Rem<Output = Self>
    + 'static
{
    /// A uint of the same width as the float
    type Int: Int<OtherSign = Self::SignedInt, UnsignedInt = Self::Int>;

    /// A int of the same width as the float
    type SignedInt: Int + MinInt<OtherSign = Self::Int, UnsignedInt = Self::Int>;

    /// An int capable of containing the exponent bits plus a sign bit. This is signed.
    type ExpInt: Int;

    const ZERO: Self;
    const ONE: Self;

    /// The bitwidth of the float type
    const BITS: u32;

    /// The bitwidth of the significand
    const SIGNIFICAND_BITS: u32;

    /// The bitwidth of the exponent
    const EXPONENT_BITS: u32 = Self::BITS - Self::SIGNIFICAND_BITS - 1;

    /// The saturated value of the exponent (infinite representation), in the rightmost postiion.
    const EXPONENT_MAX: u32 = (1 << Self::EXPONENT_BITS) - 1;

    /// The exponent bias value
    const EXPONENT_BIAS: u32 = Self::EXPONENT_MAX >> 1;

    /// A mask for the sign bit
    const SIGN_MASK: Self::Int;

    /// A mask for the significand
    const SIGNIFICAND_MASK: Self::Int;

    /// The implicit bit of the float format
    const IMPLICIT_BIT: Self::Int;

    /// A mask for the exponent
    const EXPONENT_MASK: Self::Int;

    /// Returns `self` transmuted to `Self::Int`
    fn to_bits(self) -> Self::Int;

    /// Returns `self` transmuted to `Self::SignedInt`
    fn signed_repr(self) -> Self::SignedInt;

    /// Checks if two floats have the same bit representation. *Except* for NaNs! NaN can be
    /// represented in multiple different ways. This method returns `true` if two NaNs are
    /// compared.
    fn eq_repr(self, rhs: Self) -> bool;

    /// Returns true if the sign is negative
    fn is_sign_negative(self) -> bool;

    /// Returns the exponent, not adjusting for bias.
    fn exp(self) -> Self::ExpInt;

    /// Returns the significand with no implicit bit (or the "fractional" part)
    fn frac(self) -> Self::Int;

    /// Returns the significand with implicit bit
    fn imp_frac(self) -> Self::Int;

    /// Returns a `Self::Int` transmuted back to `Self`
    fn from_bits(a: Self::Int) -> Self;

    /// Constructs a `Self` from its parts. Inputs are treated as bits and shifted into position.
    fn from_parts(sign: bool, exponent: Self::Int, significand: Self::Int) -> Self;

    /// Returns (normalized exponent, normalized significand)
    fn normalize(significand: Self::Int) -> (i32, Self::Int);

    /// Returns if `self` is subnormal
    fn is_subnormal(self) -> bool;
}

macro_rules! float_impl {
    ($ty:ident, $ity:ident, $sity:ident, $expty:ident, $bits:expr, $significand_bits:expr) => {
        impl Float for $ty {
            type Int = $ity;
            type SignedInt = $sity;
            type ExpInt = $expty;

            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;

            const BITS: u32 = $bits;
            const SIGNIFICAND_BITS: u32 = $significand_bits;

            const SIGN_MASK: Self::Int = 1 << (Self::BITS - 1);
            const SIGNIFICAND_MASK: Self::Int = (1 << Self::SIGNIFICAND_BITS) - 1;
            const IMPLICIT_BIT: Self::Int = 1 << Self::SIGNIFICAND_BITS;
            const EXPONENT_MASK: Self::Int = !(Self::SIGN_MASK | Self::SIGNIFICAND_MASK);

            fn to_bits(self) -> Self::Int {
                self.to_bits()
            }
            fn signed_repr(self) -> Self::SignedInt {
                self.to_bits() as Self::SignedInt
            }
            fn eq_repr(self, rhs: Self) -> bool {
                fn is_nan(x: $ty) -> bool {
                    x.is_nan()
                }
                if is_nan(self) && is_nan(rhs) {
                    true
                } else {
                    self.to_bits() == rhs.to_bits()
                }
            }
            fn is_sign_negative(self) -> bool {
                self.is_sign_negative()
            }
            fn exp(self) -> Self::ExpInt {
                ((self.to_bits() & Self::EXPONENT_MASK) >> Self::SIGNIFICAND_BITS) as Self::ExpInt
            }
            fn frac(self) -> Self::Int {
                self.to_bits() & Self::SIGNIFICAND_MASK
            }
            fn imp_frac(self) -> Self::Int {
                self.frac() | Self::IMPLICIT_BIT
            }
            fn from_bits(a: Self::Int) -> Self {
                Self::from_bits(a)
            }
            fn from_parts(sign: bool, exponent: Self::Int, significand: Self::Int) -> Self {
                Self::from_bits(
                    ((sign as Self::Int) << (Self::BITS - 1))
                        | ((exponent << Self::SIGNIFICAND_BITS) & Self::EXPONENT_MASK)
                        | (significand & Self::SIGNIFICAND_MASK),
                )
            }
            fn normalize(significand: Self::Int) -> (i32, Self::Int) {
                let shift = significand
                    .leading_zeros()
                    .wrapping_sub(Self::EXPONENT_BITS);
                (
                    1i32.wrapping_sub(shift as i32),
                    significand << shift as Self::Int,
                )
            }
            fn is_subnormal(self) -> bool {
                (self.to_bits() & Self::EXPONENT_MASK) == Self::Int::ZERO
            }
        }
    };
}

// #[cfg(f16_enabled)]
// float_impl!(f16, u16, i16, i8, 16, 10);
float_impl!(f32, u32, i32, i16, 32, 23);
float_impl!(f64, u64, i64, i16, 64, 52);
// #[cfg(f128_enabled)]
// float_impl!(f128, u128, i128, i16, 128, 112);

/// Minimal integer implementations needed on all integer types, including wide integers.
#[allow(dead_code)]
pub(crate) trait MinInt:
    Copy
    + core::fmt::Debug
    + ops::BitOr<Output = Self>
    + ops::Not<Output = Self>
    + ops::Shl<u32, Output = Self>
{
    /// Type with the same width but other signedness
    type OtherSign: MinInt;
    /// Unsigned version of Self
    type UnsignedInt: MinInt;

    /// If `Self` is a signed integer
    const SIGNED: bool;

    /// The bitwidth of the int type
    const BITS: u32;

    const ZERO: Self;
    const ONE: Self;
    const MIN: Self;
    const MAX: Self;
}

/// Trait for some basic operations on integers
#[allow(dead_code)]
pub(crate) trait Int:
    MinInt
    + PartialEq
    + PartialOrd
    + ops::AddAssign
    + ops::SubAssign
    + ops::BitAndAssign
    + ops::BitOrAssign
    + ops::BitXorAssign
    + ops::ShlAssign<i32>
    + ops::ShlAssign<u32>
    + ops::ShrAssign<u32>
    + ops::Add<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Mul<Output = Self>
    + ops::Div<Output = Self>
    + ops::Shr<u32, Output = Self>
    + ops::BitXor<Output = Self>
    + ops::BitAnd<Output = Self>
{
    fn unsigned(self) -> Self::UnsignedInt;
    fn from_unsigned(unsigned: Self::UnsignedInt) -> Self;

    fn from_bool(b: bool) -> Self;

    /// Prevents the need for excessive conversions between signed and unsigned
    fn logical_shr(self, other: u32) -> Self;

    /// Absolute difference between two integers.
    fn abs_diff(self, other: Self) -> Self::UnsignedInt;

    // copied from primitive integers, but put in a trait
    fn is_zero(self) -> bool;
    fn wrapping_neg(self) -> Self;
    fn wrapping_add(self, other: Self) -> Self;
    fn wrapping_mul(self, other: Self) -> Self;
    fn wrapping_sub(self, other: Self) -> Self;
    fn wrapping_shl(self, other: u32) -> Self;
    fn wrapping_shr(self, other: u32) -> Self;
    fn rotate_left(self, other: u32) -> Self;
    fn overflowing_add(self, other: Self) -> (Self, bool);
    fn leading_zeros(self) -> u32;
    fn ilog2(self) -> u32;
}

macro_rules! int_impl_common {
    ($ty:ty) => {
        fn from_bool(b: bool) -> Self {
            b as $ty
        }

        fn logical_shr(self, other: u32) -> Self {
            Self::from_unsigned(self.unsigned().wrapping_shr(other))
        }

        fn is_zero(self) -> bool {
            self == Self::ZERO
        }

        fn wrapping_neg(self) -> Self {
            <Self>::wrapping_neg(self)
        }

        fn wrapping_add(self, other: Self) -> Self {
            <Self>::wrapping_add(self, other)
        }

        fn wrapping_mul(self, other: Self) -> Self {
            <Self>::wrapping_mul(self, other)
        }

        fn wrapping_sub(self, other: Self) -> Self {
            <Self>::wrapping_sub(self, other)
        }

        fn wrapping_shl(self, other: u32) -> Self {
            <Self>::wrapping_shl(self, other)
        }

        fn wrapping_shr(self, other: u32) -> Self {
            <Self>::wrapping_shr(self, other)
        }

        fn rotate_left(self, other: u32) -> Self {
            <Self>::rotate_left(self, other)
        }

        fn overflowing_add(self, other: Self) -> (Self, bool) {
            <Self>::overflowing_add(self, other)
        }

        fn leading_zeros(self) -> u32 {
            <Self>::leading_zeros(self)
        }

        fn ilog2(self) -> u32 {
            <Self>::ilog2(self)
        }
    };
}

macro_rules! int_impl {
    ($ity:ty, $uty:ty) => {
        impl MinInt for $uty {
            type OtherSign = $ity;
            type UnsignedInt = $uty;

            const BITS: u32 = <Self as MinInt>::ZERO.count_zeros();
            const SIGNED: bool = Self::MIN != Self::ZERO;

            const ZERO: Self = 0;
            const ONE: Self = 1;
            const MIN: Self = <Self>::MIN;
            const MAX: Self = <Self>::MAX;
        }

        impl Int for $uty {
            fn unsigned(self) -> $uty {
                self
            }

            // It makes writing macros easier if this is implemented for both signed and unsigned
            #[allow(clippy::wrong_self_convention)]
            fn from_unsigned(me: $uty) -> Self {
                me
            }

            fn abs_diff(self, other: Self) -> Self {
                if self < other {
                    other.wrapping_sub(self)
                } else {
                    self.wrapping_sub(other)
                }
            }

            int_impl_common!($uty);
        }

        impl MinInt for $ity {
            type OtherSign = $uty;
            type UnsignedInt = $uty;

            const BITS: u32 = <Self as MinInt>::ZERO.count_zeros();
            const SIGNED: bool = Self::MIN != Self::ZERO;

            const ZERO: Self = 0;
            const ONE: Self = 1;
            const MIN: Self = <Self>::MIN;
            const MAX: Self = <Self>::MAX;
        }

        impl Int for $ity {
            fn unsigned(self) -> $uty {
                self as $uty
            }

            fn from_unsigned(me: $uty) -> Self {
                me as $ity
            }

            fn abs_diff(self, other: Self) -> $uty {
                self.wrapping_sub(other).wrapping_abs() as $uty
            }

            int_impl_common!($ity);
        }
    };
}

int_impl!(isize, usize);
int_impl!(i8, u8);
int_impl!(i16, u16);
int_impl!(i32, u32);
int_impl!(i64, u64);
int_impl!(i128, u128);

/// Trait for integers twice the bit width of another integer. This is implemented for all
/// primitives except for `u8`, because there is not a smaller primitive.
pub(crate) trait DInt: MinInt {
    /// Integer that is half the bit width of the integer this trait is implemented for
    type H: HInt<D = Self>;

    /// Returns the low half of `self`
    fn lo(self) -> Self::H;
    /// Returns the high half of `self`
    fn hi(self) -> Self::H;
    /// Returns the low and high halves of `self` as a tuple
    fn lo_hi(self) -> (Self::H, Self::H) {
        (self.lo(), self.hi())
    }
    /// Constructs an integer using lower and higher half parts
    fn from_lo_hi(lo: Self::H, hi: Self::H) -> Self {
        lo.zero_widen() | hi.widen_hi()
    }
}

/// Trait for integers half the bit width of another integer. This is implemented for all
/// primitives except for `u128`, because it there is not a larger primitive.
pub(crate) trait HInt: Int {
    /// Integer that is double the bit width of the integer this trait is implemented for
    type D: DInt<H = Self> + MinInt;

    // NB: some of the below methods could have default implementations (e.g. `widen_hi`), but for
    // unknown reasons this can cause infinite recursion when optimizations are disabled. See
    // <https://github.com/rust-lang/compiler-builtins/pull/707> for context.

    /// Widens (using default extension) the integer to have double bit width
    fn widen(self) -> Self::D;
    /// Widens (zero extension only) the integer to have double bit width. This is needed to get
    /// around problems with associated type bounds (such as `Int<Othersign: DInt>`) being unstable
    fn zero_widen(self) -> Self::D;
    /// Widens the integer to have double bit width and shifts the integer into the higher bits
    fn widen_hi(self) -> Self::D;
    /// Widening multiplication with zero widening. This cannot overflow.
    fn zero_widen_mul(self, rhs: Self) -> Self::D;
    /// Widening multiplication. This cannot overflow.
    fn widen_mul(self, rhs: Self) -> Self::D;
}

macro_rules! impl_d_int {
    ($($X:ident $D:ident),*) => {
        $(
            impl DInt for $D {
                type H = $X;

                fn lo(self) -> Self::H {
                    self as $X
                }
                fn hi(self) -> Self::H {
                    (self >> <$X as MinInt>::BITS) as $X
                }
            }
        )*
    };
}

macro_rules! impl_h_int {
    ($($H:ident $uH:ident $X:ident),*) => {
        $(
            impl HInt for $H {
                type D = $X;

                fn widen(self) -> Self::D {
                    self as $X
                }
                fn zero_widen(self) -> Self::D {
                    (self as $uH) as $X
                }
                fn zero_widen_mul(self, rhs: Self) -> Self::D {
                    self.zero_widen().wrapping_mul(rhs.zero_widen())
                }
                fn widen_mul(self, rhs: Self) -> Self::D {
                    self.widen().wrapping_mul(rhs.widen())
                }
                fn widen_hi(self) -> Self::D {
                    (self as $X) << <Self as MinInt>::BITS
                }
            }
        )*
    };
}

impl_d_int!(u8 u16, u16 u32, u32 u64, u64 u128, i8 i16, i16 i32, i32 i64, i64 i128);
impl_h_int!(
    u8 u8 u16,
    u16 u16 u32,
    u32 u32 u64,
    u64 u64 u128,
    i8 u8 i16,
    i16 u16 i32,
    i32 u32 i64,
    i64 u64 i128
);

/// Trait to express (possibly lossy) casting of integers
pub(crate) trait CastInto<T: Copy>: Copy {
    fn cast(self) -> T;
}

pub(crate) trait CastFrom<T: Copy>: Copy {
    fn cast_from(value: T) -> Self;
}

impl<T: Copy, U: CastInto<T> + Copy> CastFrom<U> for T {
    fn cast_from(value: U) -> Self {
        value.cast()
    }
}

macro_rules! cast_into {
    ($ty:ty) => {
        cast_into!($ty; usize, isize, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);
    };
    ($ty:ty; $($into:ty),*) => {$(
        impl CastInto<$into> for $ty {
            fn cast(self) -> $into {
                self as $into
            }
        }
    )*};
}

cast_into!(usize);
cast_into!(isize);
cast_into!(u8);
cast_into!(i8);
cast_into!(u16);
cast_into!(i16);
cast_into!(u32);
cast_into!(i32);
cast_into!(u64);
cast_into!(i64);
cast_into!(u128);
cast_into!(i128);
