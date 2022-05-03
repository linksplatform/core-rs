use crate::{Flow, LinksConstants};
use num_traits::zero;
use platform_num::LinkType;
use std::error::Error;

pub type ReadHandler<'a, T> = &'a mut dyn FnMut(&[T]) -> Flow;

pub type WriteHandler<'a, T> = &'a mut dyn FnMut(&[T], &[T]) -> Flow;

pub trait Links<T: LinkType> {
    fn constants_links(&self) -> LinksConstants<T>;

    fn count_links(&self, query: &[T]) -> T;

    fn create_links(
        &mut self,
        query: &[T],
        handler: WriteHandler<T>,
    ) -> Result<Flow, Box<dyn Error>>;

    fn each_links(&self, query: &[T], handler: ReadHandler<T>) -> Result<Flow, Box<dyn Error>>;

    fn update_links(
        &mut self,
        query: &[T],
        replacement: &[T],
        handler: WriteHandler<T>,
    ) -> Result<Flow, Box<dyn Error>>;

    fn delete_links(
        &mut self,
        query: &[T],
        handler: WriteHandler<T>,
    ) -> Result<Flow, Box<dyn Error>>;
}
