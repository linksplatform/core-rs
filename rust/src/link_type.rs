use funty::Unsigned;
use std::convert::TryFrom;
use std::hint;
use std::marker::Destruct;
use std::ops::Div;

/// # Safety
/// By default implementation this trait can be used only with [`Unsigned`] types.
///
/// [`Unsigned`]: funty::Unsigned
pub unsafe trait FuntyPart: Sized + TryFrom<u8> {
    fn funty(n: u8) -> Self;
}

// SAFETY: later it will impl only for `Unsigned` (u8..u128)
unsafe impl<All: TryFrom<u8>> const FuntyPart for All {
    fn funty(n: u8) -> Self
    where
        All: ~const Destruct,
        <All as TryFrom<u8>>::Error: ~const Destruct,
    {
        match All::try_from(n) {
            Ok(all) => all,
            Err(_) => {
                // SAFETY: T is unsigned and u8 is always less than T::MAX.
                unsafe { hint::unreachable_unchecked() }
            }
        }
    }
}

pub trait LinkType: Unsigned + FuntyPart {}

impl<All: Unsigned + FuntyPart> const LinkType for All where All: ~const Div {}
