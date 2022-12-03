use crate::{Hybrid, LinkType};

#[derive(Default)]
#[deprecated(note = "maybe deprecated")]
pub struct AddrToRaw;

impl AddrToRaw {
    pub fn convert<T: LinkType>(&self, source: T) -> T {
        Hybrid::external(source).as_inner()
    }
}

#[derive(Default)]
#[deprecated(note = "maybe deprecated")]
pub struct RawToAddr;

impl RawToAddr {
    pub fn convert<T: LinkType>(&self, source: T) -> T {
        Hybrid::external(source).abs()
    }
}
