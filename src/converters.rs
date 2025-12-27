use crate::{Hybrid, LinkType};

#[derive(Default)]
pub struct AddrToRaw;

impl AddrToRaw {
    pub fn convert<T: LinkType>(&self, source: T) -> T {
        Hybrid::external(source).as_inner()
    }
}

#[derive(Default)]
pub struct RawToAddr;

impl RawToAddr {
    pub fn convert<T: LinkType>(&self, source: T) -> T {
        Hybrid::external(source).abs()
    }
}
