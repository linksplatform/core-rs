use platform_data::{AddrToRaw, Hybrid, RawToAddr};
use quickcheck_macros::quickcheck;

#[quickcheck]
fn basic(orig: usize) -> bool {
    RawToAddr.convert(AddrToRaw.convert(orig)) == orig && Hybrid::new(orig).abs() == orig
}
