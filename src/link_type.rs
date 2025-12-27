use funty::Unsigned;
use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
};

/// Helper trait for converting from small unsigned integers.
///
/// This trait provides a convenient way to convert `u8` values
/// to any type that implements `TryFrom<u8>`.
pub trait FuntyPart: Sized + TryFrom<u8> {
    /// Convert a `u8` value to `Self`.
    ///
    /// # Panics
    ///
    /// This should never panic for unsigned integer types as they all
    /// implement `TryFrom<u8>` with `Error = Infallible`.
    fn funty(n: u8) -> Self;
}

// TryFrom<u8> has `Error = Infallible` for all unsigned integer types
impl<T> FuntyPart for T
where
    T: TryFrom<u8>,
    <T as TryFrom<u8>>::Error: Debug,
{
    fn funty(n: u8) -> Self {
        T::try_from(n).expect("conversion from u8 should never fail for unsigned types")
    }
}

/// Trait for types that can be used as link identifiers.
///
/// This trait bounds the type to be an unsigned integer with
/// various conversion capabilities.
pub trait LinkType:
    Unsigned
    + FuntyPart
    + TryFrom<i8, Error: Debug>
    + TryFrom<u8, Error: Debug>
    + TryFrom<i16, Error: Debug>
    + TryFrom<u16, Error: Debug>
    + TryFrom<i32, Error: Debug>
    + TryFrom<u32, Error: Debug>
    + TryFrom<i64, Error: Debug>
    + TryFrom<u64, Error: Debug>
    + TryFrom<i128, Error: Debug>
    + TryFrom<u128, Error: Debug>
    + TryFrom<isize, Error: Debug>
    + TryFrom<usize, Error: Debug>
    + TryInto<i8, Error: Debug>
    + TryInto<u8, Error: Debug>
    + TryInto<i16, Error: Debug>
    + TryInto<u16, Error: Debug>
    + TryInto<i32, Error: Debug>
    + TryInto<u32, Error: Debug>
    + TryInto<i64, Error: Debug>
    + TryInto<u64, Error: Debug>
    + TryInto<i128, Error: Debug>
    + TryInto<u128, Error: Debug>
    + TryInto<isize, Error: Debug>
    + TryInto<usize, Error: Debug>
{
}

impl<T> LinkType for T where
    T: Unsigned
        + FuntyPart
        + TryFrom<i8, Error: Debug>
        + TryFrom<u8, Error: Debug>
        + TryFrom<i16, Error: Debug>
        + TryFrom<u16, Error: Debug>
        + TryFrom<i32, Error: Debug>
        + TryFrom<u32, Error: Debug>
        + TryFrom<i64, Error: Debug>
        + TryFrom<u64, Error: Debug>
        + TryFrom<i128, Error: Debug>
        + TryFrom<u128, Error: Debug>
        + TryFrom<isize, Error: Debug>
        + TryFrom<usize, Error: Debug>
        + TryInto<i8, Error: Debug>
        + TryInto<u8, Error: Debug>
        + TryInto<i16, Error: Debug>
        + TryInto<u16, Error: Debug>
        + TryInto<i32, Error: Debug>
        + TryInto<u32, Error: Debug>
        + TryInto<i64, Error: Debug>
        + TryInto<u64, Error: Debug>
        + TryInto<i128, Error: Debug>
        + TryInto<u128, Error: Debug>
        + TryInto<isize, Error: Debug>
        + TryInto<usize, Error: Debug>
{
}
