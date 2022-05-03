use std::default::default;
use std::ops::RangeInclusive;

use crate::Hybrid;
use num_traits::{one, zero};
use platform_num::LinkType;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct LinksConstants<T: LinkType> {
    pub index_part: T,
    pub source_part: T,
    pub target_part: T,
    pub null: T,
    pub r#continue: T,
    pub r#break: T,
    pub skip: T,
    pub any: T,
    pub itself: T,
    pub error: T,
    pub internal_range: RangeInclusive<T>,
    pub external_range: Option<RangeInclusive<T>>,
}

impl<T: LinkType> LinksConstants<T> {
    fn default_target_part() -> T {
        T::one() + one()
    }

    pub fn full_new(
        target_part: T,
        internal: RangeInclusive<T>,
        external: Option<RangeInclusive<T>>,
    ) -> Self {
        // TODO: refactor this
        let one = one();
        let two = one + one;
        let three = two + one;
        let four = three + one;
        let five = four + one;
        let six = five + one;

        Self {
            index_part: zero(),
            source_part: one,
            target_part,
            null: default(),
            r#continue: *internal.end(),
            r#break: *internal.end() - one,
            skip: *internal.end() - two,
            any: *internal.end() - three,
            itself: *internal.end() - four,
            error: *internal.end() - five,
            internal_range: *internal.start()..=*internal.end() - six,
            external_range: external,
        }
    }

    // TODO: enough for now
    pub fn via_external(target_part: T, external: bool) -> Self {
        Self::full_new(
            target_part,
            Self::default_internal(external),
            Self::default_external(external),
        )
    }

    pub fn via_ranges(internal: RangeInclusive<T>, external: Option<RangeInclusive<T>>) -> Self {
        Self::full_new(Self::default_target_part(), internal, external)
    }

    pub fn via_only_external(external: bool) -> Self {
        Self::via_external(Self::default_target_part(), external)
    }

    pub fn external() -> Self {
        Self::via_only_external(true)
    }

    pub fn internal() -> Self {
        Self::via_only_external(false)
    }

    pub fn new() -> Self {
        Self::internal()
    }

    fn default_internal(external: bool) -> RangeInclusive<T> {
        if external {
            one()..=Hybrid::half()
        } else {
            one()..=T::MAX
        }
    }

    fn default_external(external: bool) -> Option<RangeInclusive<T>> {
        if external {
            Some(Hybrid::external_zero()..=T::MAX)
        } else {
            None
        }
    }

    pub fn is_internal(&self, address: T) -> bool {
        self.internal_range.contains(&address)
    }

    pub fn is_external(&self, address: T) -> bool {
        self.external_range
            .clone()
            .map_or(false, |range| range.contains(&address))
    }

    fn is_reference(&self, address: T) -> bool {
        self.is_internal(address) || self.is_external(address)
    }
}

impl<T: LinkType> Default for LinksConstants<T> {
    fn default() -> Self {
        Self::new()
    }
}
