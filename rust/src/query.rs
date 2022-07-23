use std::borrow::Cow;
use std::marker::Destruct;
use std::ops::Index;
use std::slice::SliceIndex;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Query<'a, T: Clone>(Cow<'a, [T]>);

impl<'a, T: Clone> Query<'a, T> {
    pub fn new<C>(beef: C) -> Self
    where
        C: Into<Cow<'a, [T]>>,
    {
        Query(beef.into())
    }

    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }

    pub const fn into_inner(self) -> Cow<'a, [T]>
    where
        Self: ~const Destruct,
    {
        self.0
    }

    pub fn into_owned(self) -> Vec<T> {
        self.0.into_owned()
    }
}

impl<'a, I: SliceIndex<[T]>, T: Clone> Index<I> for Query<'a, T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.0.index(index)
    }
}

pub trait ToQuery<T: Clone> {
    fn to_query(&self) -> Query<'_, T>;
}

impl<T: Clone> ToQuery<T> for Query<'_, T> {
    fn to_query(&self) -> Query<'_, T> {
        Query::new(&self[..])
    }
}

impl<T: Clone> ToQuery<T> for [T] {
    fn to_query(&self) -> Query<'_, T> {
        Query::new(self)
    }
}

impl<'a, T: Clone> ToQuery<T> for &'a [T] {
    fn to_query(&self) -> Query<'a, T> {
        Query::new(*self)
    }
}

impl<T: Clone> ToQuery<T> for Vec<T> {
    fn to_query(&self) -> Query<'_, T> {
        Query::new(&self[..])
    }
}

impl<T: Clone, const L: usize> ToQuery<T> for [T; L] {
    fn to_query(&self) -> Query<'_, T> {
        Query::new(&self[..])
    }
}

#[macro_export]
macro_rules! query {
    ($($x:expr),*) => (
        $crate::ToQuery::to_query(&[$($x),*])
    );
}
