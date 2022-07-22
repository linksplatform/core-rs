use platform_data::{AddrToRaw, Hybrid, RawToAddr};
use quickcheck_macros::quickcheck;

#[quickcheck]
fn basic(orig: usize) -> bool {
    AddrToRaw.convert(RawToAddr.convert(orig)) == orig && Hybrid::new(orig).abs() == orig
}
