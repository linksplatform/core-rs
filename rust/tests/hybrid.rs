use platform_data::{AddrToRaw, Hybrid, RawToAddr};
use quickcheck_macros::quickcheck;

#[quickcheck]
fn basic(orig: usize) -> bool {
    const HYBR: Hybrid<usize> = Hybrid::<usize>::external(usize::MAX / 2 + 123);

    RawToAddr.convert(AddrToRaw.convert(orig)) == orig && Hybrid::new(orig).abs() == orig
}
