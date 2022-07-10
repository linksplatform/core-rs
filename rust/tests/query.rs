use platform_data::{query, Query, ToQuery};

#[test]
fn by_ref() {
    let query = query![1, 2, 3];
    let ref_query = &query;
    let _: Query<_> = ref_query.to_query();
}

#[test]
fn empty_query() {
    let query: Query<usize> = query![];
    assert_eq!(query.len(), 0);
}