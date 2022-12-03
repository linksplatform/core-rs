use beef::lean::Cow;
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Query<'a, T: Clone>(Cow<'a, [T]>);

impl<'a, T: Clone> Deref for Query<'a, T> {
    type Target = Cow<'a, [T]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T: Clone> Query<'a, T> {
    pub fn new<C>(beef: C) -> Self
    where
        C: Into<Cow<'a, [T]>>,
    {
        Query(beef.into())
    }

    pub fn into_inner(self) -> Cow<'a, [T]> {
        self.0
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

impl<T: Clone> ToQuery<T> for Vec<T> {
    fn to_query(&self) -> Query<'_, T> {
        Query::new(&**self)
    }
}

impl<T: Clone, const L: usize> ToQuery<T> for [T; L] {
    fn to_query(&self) -> Query<'_, T> {
        Query::new(self.as_slice())
    }
}

#[macro_export]
macro_rules! query {
    ($($x:expr),*) => (
        $crate::ToQuery::to_query(&[$($x),*])
    );
}
