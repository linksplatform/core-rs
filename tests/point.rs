use platform_data::Point;

#[test]
fn basic() {
    let point = Point::new(228, 1337);

    assert_eq!(point.len(), 1337);

    assert_eq!(point.get(0), Some(&228));
    assert_eq!(point.get(1), Some(&228));

    assert!((0..point.len()).map(|_| 228).eq(point.into_iter()));
}

#[test]
fn test_is_empty() {
    let empty_point = Point::new(42, 0);
    assert!(empty_point.is_empty());
    assert_eq!(empty_point.len(), 0);

    let non_empty_point = Point::new(42, 5);
    assert!(!non_empty_point.is_empty());
}

#[test]
fn test_get_out_of_bounds() {
    let point = Point::new(100, 3);
    assert_eq!(point.get(0), Some(&100));
    assert_eq!(point.get(2), Some(&100));
    assert_eq!(point.get(3), None); // out of bounds
    assert_eq!(point.get(100), None); // way out of bounds
}

#[test]
fn test_is_full() {
    // Link where all parts point to the same value (full/point)
    let full_link = [5, 5, 5];
    assert!(Point::<i32>::is_full(&full_link));

    // Link where parts differ (not full)
    let not_full = [1, 2, 3];
    assert!(!Point::<i32>::is_full(&not_full));

    // Edge case: two elements, same value
    let two_same = [7, 7];
    assert!(Point::<i32>::is_full(&two_same));

    // Edge case: two elements, different values
    let two_different = [7, 8];
    assert!(!Point::<i32>::is_full(&two_different));
}

#[test]
fn test_is_partial() {
    // Link where at least one part points to index (partial)
    let partial = [1, 1, 2];
    assert!(Point::<i32>::is_partial(&partial));

    // Link where no part equals the first one (not partial)
    let not_partial = [1, 2, 3];
    assert!(!Point::<i32>::is_partial(&not_partial));

    // Full link is also partial (all equal first)
    let full_link = [5, 5, 5];
    assert!(Point::<i32>::is_partial(&full_link));

    // Edge case: two elements, same
    let two_same = [7, 7];
    assert!(Point::<i32>::is_partial(&two_same));
}

#[test]
#[should_panic(expected = "cannot determine link's pointless using only its identifier")]
fn test_is_full_panics_with_single_element() {
    let single = [1];
    Point::<i32>::is_full(&single);
}

#[test]
#[should_panic(expected = "cannot determine link's pointless using only its identifier")]
fn test_is_partial_panics_with_single_element() {
    let single = [1];
    Point::<i32>::is_partial(&single);
}

#[test]
fn test_into_iter() {
    let point = Point::new(42, 5);
    let collected: Vec<i32> = point.into_iter().collect();
    assert_eq!(collected, vec![42, 42, 42, 42, 42]);
}

#[test]
fn test_into_iter_empty() {
    let point = Point::new(42, 0);
    assert!(point.into_iter().next().is_none());
}
