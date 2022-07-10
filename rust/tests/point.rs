use platform_data::Point;

#[test]
fn basic() {
    let point = Point::new(228, 1337);

    assert_eq!(point.len(), 1337);

    assert_eq!(point.get(0), Some(&228));
    assert_eq!(point.get(1), Some(&228));

    assert!((0..point.len()).map(|_| 228).eq(point.into_iter()));
}
