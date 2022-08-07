use crate::{Flow, LinkType, LinksConstants};
use std::{borrow::Cow, error, io};

#[derive(thiserror::Error, Debug)]
pub enum Error<'a, T: LinkType> {
    #[error("link {0} does not exist.")]
    NotExists(T),

    #[error("link {0:?} has dependencies")]
    HasUsages(Vec<Cow<'a, [T]>>),

    #[error("link {0} already exists")]
    AlreadyExists(Cow<'a, T>),

    #[error("limit for the number of links in the storage has been reached: {0}")]
    LimitReached(T),

    #[error("unable to allocate memory for links storage: `{0}`")]
    AllocFailed(#[from] io::Error),

    #[error("other internal error: `{0}`")]
    Other(#[from] Box<dyn error::Error + Sync + Send>),
}

pub type ReadHandler<'a, T> = &'a mut dyn FnMut(&[T]) -> Flow;

pub type WriteHandler<'a, T> = &'a mut dyn FnMut(&[T], &[T]) -> Flow;

pub trait Links<T: LinkType> {
    fn constants_links(&self) -> LinksConstants<T>;

    fn count_links(&self, query: &[T]) -> T;

    fn create_links(&mut self, query: &[T], handler: WriteHandler<T>)
    -> Result<Flow, Error<'_, T>>;

    fn each_links(&self, query: &[T], handler: ReadHandler<T>) -> Result<Flow, Error<'_, T>>;

    fn update_links(
        &mut self,
        query: &[T],
        replacement: &[T],
        handler: WriteHandler<T>,
    ) -> Result<Flow, Error<'_, T>>;

    fn delete_links(&mut self, query: &[T], handler: WriteHandler<T>)
    -> Result<Flow, Error<'_, T>>;
}
