use crate::{Flow, LinkType};
use std::{borrow::Cow, error, io};

type Repr<T> = Box<[T]>;

#[derive(thiserror::Error, Debug)]
pub enum Error<T: LinkType> {
    #[error("link `{0}` does not exist")]
    NotExists(T),

    #[error("link `{0:?}` cannot change link because it is also has usages")]
    HasUsages(Box<[Repr<T>]>),

    #[error("link `{0}` already exists")]
    AlreadyExists(Repr<T>),

    #[error("limit for the number of links in the storage has been reached: `{0}`")]
    LimitReached(T),

    #[error("unable to allocate memory for links storage: `{0}`")]
    AllocFailed(#[from] io::Error),

    #[error("other internal error: `{0}`")]
    Other(#[from] Box<dyn error::Error + Sync + Send>),
}

fn _assert_send_sync<T: LinkType>() {
    fn _assert_send_sync<T: Send + Sync>() {}
    _assert_send_sync::<Error<T>>();
}

pub type ReadHandler<'a, T> = &'a mut dyn FnMut(&[T]) -> Flow;

pub type WriteHandler<'a, T> = &'a mut dyn FnMut(&[T], &[T]) -> Flow;

pub trait Links<T: LinkType> {
    fn count_links(&self, query: &[T]) -> T;

    fn create_links(&mut self, query: &[T], handler: WriteHandler<'_, T>)
    -> Result<Flow, Error<T>>;

    fn each_links(&self, query: &[T], handler: ReadHandler<'_, T>) -> Result<Flow, Error<T>>;

    fn update_links(
        &mut self,
        query: &[T],
        replacement: &[T],
        handler: WriteHandler<'_, T>,
    ) -> Result<Flow, Error<T>>;

    fn delete_links(&mut self, query: &[T], handler: WriteHandler<'_, T>)
    -> Result<Flow, Error<T>>;
}
