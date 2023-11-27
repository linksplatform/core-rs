#![feature(try_trait_v2)]

mod link;
mod links;

pub use {
    link::{Flow, Link},
    links::Error,
};

use std::fmt;

pub unsafe trait LinkType: Copy + fmt::Debug + Sync + Send {
    fn from_addr(addr: usize) -> Self;
    fn addr(self) -> usize;
}

macro_rules! link_type {
    ($($ty:ty)*) => {$(
        unsafe impl LinkType for $ty {
            #[inline(always)]
            fn from_addr(addr: usize) -> Self {
                addr as Self
            }

            #[inline(always)]
            fn addr(self) -> usize {
                self as usize
            }
        }
    )*};
}

link_type! { u8 u16 u32 u64 usize }
