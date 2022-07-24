use crate::{Hybrid, LinkType};
use funty::Integral;
use std::ops::Sub;

#[derive(Default)]
pub struct AddrToRaw;

impl AddrToRaw {
    pub const fn convert<T: LinkType + ~const Integral + ~const Sub>(&self, source: T) -> T {
        Hybrid::external(source).as_inner()
    }
}

#[derive(Default)]
pub struct RawToAddr;

impl RawToAddr {
    pub const fn convert<T: LinkType + ~const Integral + ~const Sub>(&self, source: T) -> T {
        Hybrid::external(source).abs()
    }
}
