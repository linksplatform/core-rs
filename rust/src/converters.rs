use crate::Hybrid;
use crate::LinkType;

#[derive(Default)]
pub struct AddrToRaw;

impl AddrToRaw {
    pub fn new() -> Self {
        Self
    }

    pub fn convert<T: LinkType>(&self, source: T) -> T {
        Hybrid::new(source).as_inner()
    }
}

#[derive(Default)]
pub struct RawToAddr;

impl RawToAddr {
    pub fn new() -> Self {
        Self
    }

    pub fn convert<T: LinkType>(&self, source: T) -> T {
        Hybrid::new(source).abs()
    }
}
