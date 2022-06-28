use platform_num::LinkType;
use std::borrow::{Borrow, Cow};
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

    pub fn len(&self) -> usize {
        match self.0 {
            Cow::Borrowed(beef) => beef.len(),
            Cow::Owned(ref beef) => beef.len(),
        }
    }

    pub fn into_inner(self) -> Cow<'a, [T]> {
        self.0
    }

    pub fn into_owned(self) -> Vec<T> {
        match self.0 {
            Cow::Borrowed(beef) => beef.to_owned(),
            Cow::Owned(beef) => beef,
        }
    }
}

impl<'a, I: SliceIndex<[T]>, T: Clone> Index<I> for Query<'a, T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        match self.0 {
            Cow::Borrowed(ref s) => &s[index],
            Cow::Owned(ref s) => &s[index],
        }
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
    () => (
        $crate::Query::new(&[][..])
    );
    ($($x:expr),*) => (
        $crate::Query::new(&[$($x),*][..])
    );
}
