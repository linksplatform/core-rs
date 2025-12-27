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

#[test]
fn test_is_empty() {
    let empty_query: Query<i32> = query![];
    assert!(empty_query.is_empty());

    let non_empty: Query<i32> = query![1, 2, 3];
    assert!(!non_empty.is_empty());
}

#[test]
fn test_len() {
    let query: Query<i32> = query![1, 2, 3, 4, 5];
    assert_eq!(query.len(), 5);
}

#[test]
fn test_into_inner() {
    let query: Query<i32> = query![10, 20, 30];
    let cow = query.into_inner();
    assert_eq!(cow.len(), 3);
}

#[test]
fn test_into_owned() {
    let query: Query<i32> = query![10, 20, 30];
    let owned: Vec<i32> = query.into_owned();
    assert_eq!(owned, vec![10, 20, 30]);
}

#[test]
fn test_index() {
    let query: Query<i32> = query![10, 20, 30];
    assert_eq!(query[0], 10);
    assert_eq!(query[1], 20);
    assert_eq!(query[2], 30);
}

#[test]
fn test_slice_to_query() {
    let slice: &[i32] = &[1, 2, 3];
    let query = slice.to_query();
    assert_eq!(query.len(), 3);
    assert_eq!(query[0], 1);
}

#[test]
fn test_vec_to_query() {
    let vec = vec![1, 2, 3, 4];
    let query = vec.to_query();
    assert_eq!(query.len(), 4);
    assert_eq!(query[0], 1);
}

#[test]
fn test_array_to_query() {
    let array = [1, 2, 3, 4, 5];
    let query = array.to_query();
    assert_eq!(query.len(), 5);
    assert_eq!(query[0], 1);
}

#[test]
fn test_query_clone() {
    let query: Query<i32> = query![1, 2, 3];
    let cloned = query.clone();
    assert_eq!(query, cloned);
}

#[test]
fn test_query_debug() {
    let query: Query<i32> = query![1, 2, 3];
    let debug_str = format!("{:?}", query);
    assert!(debug_str.contains("Query"));
}

#[test]
fn test_query_equality() {
    let query1: Query<i32> = query![1, 2, 3];
    let query2: Query<i32> = query![1, 2, 3];
    let query3: Query<i32> = query![1, 2, 4];

    assert_eq!(query1, query2);
    assert_ne!(query1, query3);
}

#[test]
fn test_borrowed_slice_ref_to_query() {
    let data = [1, 2, 3];
    let slice_ref: &[i32] = &data;
    let query = slice_ref.to_query();
    assert_eq!(query.len(), 3);
}

#[test]
fn test_into_inner_explicit() {
    // Test into_inner specifically returns the inner Cow
    let query: Query<i32> = Query::new(vec![1, 2, 3]);
    let inner = query.into_inner();
    // Verify we can access the inner data
    assert_eq!(inner.len(), 3);
    assert_eq!(inner[0], 1);
}

#[test]
fn test_unsized_slice_to_query() {
    // Test ToQuery for [T] - dereference to unsized slice
    fn test_slice_impl(slice: &[i32]) -> Query<'_, i32> {
        <[i32] as ToQuery<i32>>::to_query(slice)
    }
    let data = [1, 2, 3];
    let query = test_slice_impl(&data);
    assert_eq!(query.len(), 3);
}

#[test]
fn test_ref_slice_to_query_explicit() {
    // Explicitly test &[T] implementation
    fn test_ref_slice_impl<'a>(slice: &'a &'a [i32]) -> Query<'a, i32> {
        ToQuery::<i32>::to_query(slice)
    }
    let data = [10, 20, 30];
    let slice_ref: &[i32] = &data;
    let query = test_ref_slice_impl(&slice_ref);
    assert_eq!(query.len(), 3);
    assert_eq!(query[0], 10);
}
