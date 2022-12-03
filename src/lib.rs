#![forbid(rust_2018_idioms)]
#![deny(clippy::all, clippy::perf)]
#![warn(clippy::nursery)]
#![feature(
    try_trait_v2,
    associated_type_bounds,
    type_alias_impl_trait,
    const_refs_to_cell,
    const_result_drop,
    const_trait_impl
)]

mod converters;
mod flow;
mod hybrid;
mod link_type;
mod links;
mod point;
mod query;

pub use converters::{AddrToRaw, RawToAddr};
pub use flow::Flow;
pub use hybrid::Hybrid;
pub use link_type::LinkType;
pub use links::{Error, Links, ReadHandler, WriteHandler};
pub use point::Point;
pub use query::{Query, ToQuery};
