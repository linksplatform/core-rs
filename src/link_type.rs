use funty::Unsigned;
use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
};

/// Trait for creating small numeric values from u8.
///
/// This trait provides a convenient way to create numeric types from small u8 values,
/// which is commonly needed when working with link constants and indices.
pub trait FuntyPart: Sized + TryFrom<u8> {
    /// Create a value from a u8. Panics if the conversion fails.
    fn funty(n: u8) -> Self;
}

impl<All: TryFrom<u8>> FuntyPart for All
where
    <All as TryFrom<u8>>::Error: Debug,
{
    fn funty(n: u8) -> Self {
        All::try_from(n).expect("conversion from u8 should succeed for all unsigned types")
    }
}

/// Trait bound for numeric types that can be used as link identifiers.
///
/// This trait combines several requirements:
/// - Must be an unsigned integer type (via `Unsigned`)
/// - Must support creation from small values (via `FuntyPart`)
/// - Must support conversions to/from various integer types
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

impl<All: Unsigned + FuntyPart> LinkType for All where
    All: TryFrom<i8, Error: Debug>
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
