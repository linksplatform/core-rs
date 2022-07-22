use crate::LinkType;
use std::ops::Div;

#[derive(Debug, Clone, Copy, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub struct Hybrid<T> {
    value: T,
}

impl<T: LinkType> Hybrid<T> {
    pub fn new(value: T) -> Self {
        Self::internal(value)
    }

    pub const fn half() -> T
    where
        T: ~const Div<Output = T>,
    {
        T::MAX / T::funty(2)
    }

    pub fn external(value: T) -> Self {
        Self {
            value: Self::extend_value(value),
        }
    }

    pub fn internal(value: T) -> Self {
        Self { value }
    }

    fn extend_value(value: T) -> T {
        (T::MAX - value).wrapping_add(T::funty(1))
    }

    pub fn is_zero(&self) -> bool {
        self.value == T::default()
    }

    pub fn is_internal(&self) -> bool {
        self.value < Self::half() // || self.value == T::default()
    }

    pub fn is_external(&self) -> bool {
        !self.is_internal() || self.value == T::default()
    }

    pub fn abs(&self) -> T {
        self.value.wrapping_add(T::funty(1)).wrapping_add(T::MAX)
    }

    pub fn as_inner(&self) -> T {
        self.value
    }
}
