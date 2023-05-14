#![feature(try_trait_v2)]
#![feature(associated_type_bounds)]
#![feature(type_alias_impl_trait)]
#![feature(const_trait_impl)]
#![feature(step_trait)]

mod link;
mod links;

pub use {
    link::{Flow, Link},
    links::Error,
};

use std::fmt;

// fixme: track https://github.com/rust-lang/rust/issues/67792
#[const_trait]
pub unsafe trait LinkType: Copy + fmt::Debug + Sync + Send {
    fn from_addr(addr: usize) -> Self;
    fn addr(self) -> usize;
}

macro_rules! link_type {
    ($($ty:ty)*) => {$(
        unsafe impl LinkType for $ty {
            fn from_addr(addr: usize) -> Self {
                addr as Self
            }

            fn addr(self) -> usize {
                self as usize
            }
        }
    )*};
}

link_type! { u8 u16 u32 u64 usize }
