use core::ops::*;

/// [`Number`] is a marker trait for the types that reprsent an arithmetic number
/// All primitive types except `bool` and `char` implements this trait
pub trait Number:
    Sized
    + Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + core::iter::Sum
    + Default
{
    const ZERO: Self;
    const ONE: Self;
}

macro_rules! impl_number {
    ($($ty:ty);* $(;)?) => {
        $(
            impl Number for $ty {
                const ZERO: Self = 0 as _;
                const ONE: Self = 1 as _;
            }
        )*
    };
}

impl_number!(
    i8; i16; i32; i64; i128;
    u8; u16; u32; u64; u128;
    f32; f64
);
