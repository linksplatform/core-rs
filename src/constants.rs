use std::ops::{Range, Sub};

use crate::{Hybrid, LinkType};

macro_rules! const_fn {
    (const $name:ident: $ty:ty = $expr:expr; where $($bound:tt)*) => {
        const fn $name() -> $ty
        where
            $($bound)*
        {
            $expr
        }
    };
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct LinksConstants<T> {
    pub any: T,
    pub r#break: T,
    pub r#continue: T,
    pub error: T,
    pub internal: Range<T>,
    pub external: Option<Range<T>>,
}

impl<T: LinkType> LinksConstants<T> {
    const ANY: T = T::ANY;

    pub unsafe fn _new(internal: Range<T>, external: Option<Range<T>>) -> Self {
        Self {
            any: T::ANY,
            r#break: T::BREAK,
            r#continue: T::CONTINUE,
            error: T::ERROR,
            internal,
            external,
        }
    }

    // TODO: enough for now
    pub fn with_external(external: bool) -> Self {
        unsafe {
            Self::_new(
                Self::default_internal(external),
                Self::default_external(external),
            )
        }
    }

    pub fn external() -> Self {
        Self::with_external(true)
    }

    pub fn internal() -> Self {
        Self::with_external(false)
    }

    pub fn new() -> Self {
        Self::internal()
    }

    fn default_internal(external: bool) -> Range<T> {
        if external {
            T::funty(1)..Hybrid::half()
        } else {
            T::funty(1)..T::funty(223)
        }
    }

    fn default_external(external: bool) -> Option<Range<T>> {
        if external {
            Some(Hybrid::half()..T::CONST_BOUND)
        } else {
            None
        }
    }

    pub fn is_internal(&self, address: T) -> bool {
        self.internal.contains(&address)
    }

    pub fn is_external(&self, address: T) -> bool {
        self.external
            .as_ref()
            .map_or(false, |range| range.contains(&address))
    }

    pub fn is_reference(&self, address: T) -> bool {
        self.is_internal(address) || self.is_external(address)
    }
}

impl<T: LinkType> Default for LinksConstants<T> {
    fn default() -> Self {
        Self::new()
    }
}
