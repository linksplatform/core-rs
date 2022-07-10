use platform_data::Flow;

#[test]
fn basic() {
    let mut vec = vec![];

    (0..20).try_for_each(|i| {
        vec.push(i);
        if i == 10 {
            Flow::Break
        } else {
            Flow::Continue
        }
    });

    assert_eq!(vec, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}
