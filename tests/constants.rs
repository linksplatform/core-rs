use platform_data::LinksConstants;

#[test]
fn test_new_creates_internal_constants() {
    let constants: LinksConstants<u64> = LinksConstants::new();
    // new() should return internal constants (no external range)
    assert!(constants.external_range.is_none());
    assert!(constants.internal_range.start() <= constants.internal_range.end());
}

#[test]
fn test_default_creates_internal_constants() {
    let constants: LinksConstants<u64> = LinksConstants::default();
    // default() should behave like new()
    assert!(constants.external_range.is_none());
}

#[test]
fn test_internal_creates_internal_only_constants() {
    let constants: LinksConstants<u64> = LinksConstants::internal();
    assert!(constants.external_range.is_none());
    // Internal range should span from 1 to MAX
    assert_eq!(*constants.internal_range.start(), 1);
}

#[test]
fn test_external_creates_constants_with_external_range() {
    let constants: LinksConstants<u64> = LinksConstants::external();
    assert!(constants.external_range.is_some());
    let external_range = constants.external_range.as_ref().unwrap();
    assert!(external_range.start() <= external_range.end());
}

#[test]
fn test_via_only_external_false() {
    let constants: LinksConstants<u64> = LinksConstants::via_only_external(false);
    assert!(constants.external_range.is_none());
}

#[test]
fn test_via_only_external_true() {
    let constants: LinksConstants<u64> = LinksConstants::via_only_external(true);
    assert!(constants.external_range.is_some());
}

#[test]
fn test_via_external_with_custom_target_part() {
    let constants: LinksConstants<u64> = LinksConstants::via_external(5, true);
    assert_eq!(constants.target_part, 5);
    assert!(constants.external_range.is_some());
}

#[test]
fn test_via_ranges_with_custom_ranges() {
    let internal = 1u64..=1000;
    let external = Some(1001u64..=2000);
    let constants: LinksConstants<u64> =
        LinksConstants::via_ranges(internal.clone(), external);
    assert_eq!(*constants.internal_range.start(), *internal.start());
    assert!(constants.external_range.is_some());
}

#[test]
fn test_via_ranges_without_external() {
    let internal = 1u64..=1000;
    let constants: LinksConstants<u64> = LinksConstants::via_ranges(internal, None);
    assert!(constants.external_range.is_none());
}

#[test]
fn test_full_new_sets_all_fields() {
    let constants: LinksConstants<u64> = LinksConstants::full_new(3, 1..=100, Some(101..=200));
    assert_eq!(constants.index_part, 0);
    assert_eq!(constants.source_part, 1);
    assert_eq!(constants.target_part, 3);
    assert_eq!(constants.null, 0);
    // continue is the end of internal range
    assert_eq!(constants.r#continue, 100);
    // break is end - 1
    assert_eq!(constants.r#break, 99);
    // skip is end - 2
    assert_eq!(constants.skip, 98);
    // any is end - 3
    assert_eq!(constants.any, 97);
    // itself is end - 4
    assert_eq!(constants.itself, 96);
    // error is end - 5
    assert_eq!(constants.error, 95);
    // internal_range is adjusted
    assert_eq!(*constants.internal_range.start(), 1);
    assert_eq!(*constants.internal_range.end(), 94); // end - 6
    assert!(constants.external_range.is_some());
}

#[test]
fn test_is_internal_for_address_in_internal_range() {
    let constants: LinksConstants<u64> = LinksConstants::new();
    // Address within internal range should return true
    assert!(constants.is_internal(1));
    assert!(constants.is_internal(100));
}

#[test]
fn test_is_internal_for_address_outside_range() {
    let constants: LinksConstants<u64> = LinksConstants::full_new(2, 10..=100, None);
    // Address outside internal range should return false
    assert!(!constants.is_internal(5)); // below range
}

#[test]
fn test_is_external_with_no_external_range() {
    let constants: LinksConstants<u64> = LinksConstants::internal();
    // Should return false when no external range is defined
    assert!(!constants.is_external(100));
}

#[test]
fn test_is_external_with_external_range() {
    let constants: LinksConstants<u64> = LinksConstants::external();
    let external_range = constants.external_range.as_ref().unwrap();
    // Address within external range should return true
    assert!(constants.is_external(*external_range.start()));
}

#[test]
fn test_is_reference_internal() {
    let constants: LinksConstants<u64> = LinksConstants::new();
    // Internal address should be a reference
    assert!(constants.is_reference(1));
}

#[test]
fn test_is_reference_external() {
    let constants: LinksConstants<u64> = LinksConstants::external();
    let external_range = constants.external_range.as_ref().unwrap();
    // External address should be a reference
    assert!(constants.is_reference(*external_range.start()));
}

#[test]
fn test_is_reference_neither() {
    let constants: LinksConstants<u64> = LinksConstants::full_new(2, 10..=100, None);
    // Address outside both ranges should not be a reference
    assert!(!constants.is_reference(5));
}

#[test]
fn test_constants_clone() {
    let constants: LinksConstants<u64> = LinksConstants::external();
    let cloned = constants.clone();
    assert_eq!(constants, cloned);
}

#[test]
fn test_constants_debug() {
    let constants: LinksConstants<u64> = LinksConstants::new();
    let debug_str = format!("{:?}", constants);
    assert!(debug_str.contains("LinksConstants"));
}

#[test]
fn test_with_different_types() {
    // Test with u8
    let constants_u8: LinksConstants<u8> = LinksConstants::new();
    assert!(constants_u8.is_internal(1));

    // Test with u16
    let constants_u16: LinksConstants<u16> = LinksConstants::new();
    assert!(constants_u16.is_internal(1));

    // Test with u32
    let constants_u32: LinksConstants<u32> = LinksConstants::new();
    assert!(constants_u32.is_internal(1));

    // Test with usize
    let constants_usize: LinksConstants<usize> = LinksConstants::new();
    assert!(constants_usize.is_internal(1));
}

#[test]
fn test_via_external_false() {
    // Test via_external with external=false
    let constants: LinksConstants<u64> = LinksConstants::via_external(7, false);
    assert_eq!(constants.target_part, 7);
    assert!(constants.external_range.is_none());
}

#[test]
fn test_default_internal_external_ranges() {
    // Test with external mode to exercise default_external path
    let constants: LinksConstants<u64> = LinksConstants::via_external(2, true);
    // Should have external range
    assert!(constants.external_range.is_some());
    // Internal range should be limited
    assert!(*constants.internal_range.end() < u64::MAX);
}
