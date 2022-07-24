use funty::Unsigned;
use std::{
    convert::{Infallible, TryFrom},
    hint,
    marker::Destruct,
};

pub trait FuntyPart: Sized + TryFrom<u8> {
    fn funty(n: u8) -> Self;
}

// TryFrom<u8> has `Error = Infallible` for all types
impl<All: TryFrom<u8, Error = Infallible>> const FuntyPart for All {
    fn funty(n: u8) -> Self
    where
        All: ~const Destruct,
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

pub trait LinkType: Unsigned + FuntyPart {}

impl<All: Unsigned + FuntyPart> const LinkType for All {}
