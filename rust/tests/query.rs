use platform_data::{query, Query, ToQuery};

#[test]
fn by_ref() {
    let query = query![1, 2, 3];
    let ref_query = &query;
    let query = ref_query.to_query();
}
