use platform_num::LinkType;
use std::borrow::Cow;
use std::ops::Index;
use std::slice::SliceIndex;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Query<'a, T: LinkType> {
    cow: Cow<'a, [T]>,
}

impl<'a, T: LinkType> Query<'a, T> {
    pub fn new<C>(beef: C) -> Self
    where
        C: Into<Cow<'a, [T]>>,
    {
        Query { cow: beef.into() }
    }

    pub fn len(&self) -> usize {
        match self.cow {
            Cow::Borrowed(ref beef) => beef.len(),
            Cow::Owned(ref beef) => beef.len(),
        }
    }

    pub fn into_inner(self) -> Cow<'a, [T]> {
        self.cow
    }

    pub fn into_owned(self) -> Vec<T> {
        match self.cow {
            Cow::Borrowed(beef) => beef.to_vec(),
            Cow::Owned(beef) => beef,
        }
    }
}

impl<'a, I: SliceIndex<[T]>, T: LinkType> Index<I> for Query<'a, T> {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        match self.cow {
            Cow::Borrowed(ref s) => &s[index],
            Cow::Owned(ref s) => &s[index],
        }
    }
}

pub trait ToQuery<T: LinkType> {
    fn to_query(&self) -> Query<'_, T>;
}

impl<T: LinkType> ToQuery<T> for Query<'_, T> {
    fn to_query(&self) -> Query<'_, T> {
        Query::new(&self[..])
    }
}

impl<T: LinkType> ToQuery<T> for [T] {
    fn to_query(&self) -> Query<'_, T> {
        Query::new(self)
    }
}

impl<'a, T: LinkType> ToQuery<T> for &'a [T] {
    fn to_query(&self) -> Query<'a, T> {
        Query::new(*self)
    }
}

impl<T: LinkType> ToQuery<T> for Vec<T> {
    fn to_query(&self) -> Query<'_, T> {
        Query::new(&self[..])
    }
}

impl<T: LinkType, const L: usize> ToQuery<T> for [T; L] {
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
