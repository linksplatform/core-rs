use platform_data::{AddrToRaw, Hybrid, RawToAddr};
use quickcheck_macros::quickcheck;

#[quickcheck]
fn basic(orig: usize) -> bool {
    RawToAddr.convert(AddrToRaw.convert(orig)) == orig && Hybrid::new(orig).abs() == orig
}

#[test]
fn test_hybrid_new() {
    let hybrid = Hybrid::new(42usize);
    // new() creates internal hybrid
    assert!(!hybrid.is_zero());
}

#[test]
fn test_hybrid_internal() {
    let hybrid = Hybrid::<usize>::internal(100);
    assert!(hybrid.is_internal());
    assert_eq!(hybrid.as_inner(), 100);
}

#[test]
fn test_hybrid_external() {
    let hybrid = Hybrid::<usize>::external(100);
    // External values are transformed
    assert!(!hybrid.is_internal() || hybrid.is_zero());
}

#[test]
fn test_hybrid_half() {
    let half: usize = Hybrid::<usize>::half();
    // half() returns MAX/2
    assert_eq!(half, usize::MAX / 2);
}

#[test]
fn test_hybrid_is_zero() {
    let zero_hybrid = Hybrid::<usize>::internal(0);
    assert!(zero_hybrid.is_zero());

    let non_zero_hybrid = Hybrid::<usize>::internal(1);
    assert!(!non_zero_hybrid.is_zero());
}

#[test]
fn test_hybrid_is_internal() {
    // Small values should be internal
    let internal = Hybrid::<usize>::internal(100);
    assert!(internal.is_internal());
}

#[test]
fn test_hybrid_is_external() {
    // Zero is considered external
    let zero = Hybrid::<usize>::internal(0);
    assert!(zero.is_external());

    // External-created hybrids should be external
    let external = Hybrid::<usize>::external(1);
    assert!(external.is_external());
}

#[test]
fn test_hybrid_abs() {
    let hybrid = Hybrid::<usize>::internal(42);
    // For internal values, abs should return the original
    assert_eq!(hybrid.abs(), 42);
}

#[test]
fn test_hybrid_as_inner() {
    let hybrid = Hybrid::<usize>::internal(999);
    assert_eq!(hybrid.as_inner(), 999);
}

#[test]
fn test_addr_to_raw_default() {
    let converter = AddrToRaw;
    let result = converter.convert(100usize);
    // convert creates an external hybrid and returns its inner value
    assert!(result > 0);
}

#[test]
fn test_raw_to_addr_default() {
    let converter = RawToAddr;
    let result = converter.convert(100usize);
    // convert creates an external hybrid and returns its abs value
    assert!(result > 0);
}

#[test]
fn test_hybrid_copy() {
    let hybrid = Hybrid::<usize>::internal(42);
    let copied = hybrid; // Copy since Hybrid implements Copy
    assert_eq!(hybrid, copied);
}

#[test]
fn test_hybrid_debug() {
    let hybrid = Hybrid::<usize>::internal(42);
    let debug_str = format!("{:?}", hybrid);
    assert!(debug_str.contains("Hybrid"));
}

#[test]
fn test_hybrid_ord() {
    let h1 = Hybrid::<usize>::internal(10);
    let h2 = Hybrid::<usize>::internal(20);
    assert!(h1 < h2);
    assert!(h2 > h1);
}

#[test]
fn test_hybrid_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(Hybrid::<usize>::internal(42));
    set.insert(Hybrid::<usize>::internal(42));
    assert_eq!(set.len(), 1);
}

#[test]
fn test_different_types() {
    // Test with u8
    let h8 = Hybrid::<u8>::new(10);
    assert!(h8.is_internal());

    // Test with u16
    let h16 = Hybrid::<u16>::new(100);
    assert!(h16.is_internal());

    // Test with u32
    let h32 = Hybrid::<u32>::new(1000);
    assert!(h32.is_internal());

    // Test with u64
    let h64 = Hybrid::<u64>::new(10000);
    assert!(h64.is_internal());
}
