use funty::Unsigned;
use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    hint,
    marker::Destruct,
};

macro_rules! named {
    ($($name:ident => $val:expr)*) => {
        $(
            fn $name() -> Self {
                Self::funty($val)
            }
        )*
    };
}

#[const_trait]
pub trait FuntyPart: Sized + TryFrom<u8> {
    fn funty(n: u8) -> Self;

    named! {
        zero => 0
        one => 1
        two => 2
    }
}

// TryFrom<u8> has `Error = Infallible` for all types
impl<All: ~const TryFrom<u8>> const FuntyPart for All {
    fn funty(n: u8) -> Self
    where
        All: ~const Destruct,
        <All as TryFrom<u8>>::Error: ~const Destruct,
    {
        // std `Result::unwrap_unchecked` is not const
        match All::try_from(n) {
            Ok(all) => all,
            Err(_) => {
                // <All as TryFrom<u8>>::Error is Infallible
                unsafe { hint::unreachable_unchecked() }
            }
        }
    }
}

// fixme: track https://github.com/rust-lang/rust/issues/67792
#[const_trait]
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
    const ANY: Self;
}

impl<All: Unsigned + FuntyPart> const LinkType for All
where
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
        + TryInto<usize, Error: Debug>,
{
    const ANY: Self = Self::MAX;
}
