use std::{any::type_name, marker::PhantomData};
use std::fmt::Debug;


#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct WeakIndex {
    pub(super) index: usize,
}

impl Debug for WeakIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.index))
    }
}

impl WeakIndex {
    pub(super) fn new(index: usize) -> Self {
        Self { index }
    }

    pub(super) fn promote<T>(self) -> StrongIndex<T> {
        StrongIndex::new(self.index)
    }
}


#[derive(PartialEq, Eq)]
pub struct StrongIndex<T> {
    pub(super) index: usize,
    _marker: PhantomData<T>
}

impl<T> Copy for StrongIndex<T> {}

impl<T> Clone for StrongIndex<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Debug> Debug for StrongIndex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}[{}]", type_name::<T>().split("::").last().unwrap(), self.index))
    }
}

impl<T> StrongIndex<T> {
    
    #[inline]
    pub(crate) fn new(index: usize) -> StrongIndex<T> {
        StrongIndex{ 
            index, 
            _marker: PhantomData 
        }
    }

    #[inline]
    pub fn demote(self) -> WeakIndex {
        WeakIndex::new(self.index)
    }

    #[inline]
    pub fn as_weak(&self) -> WeakIndex {
        WeakIndex::new(self.index)
    }
}

impl<T> From<StrongIndex<T>> for WeakIndex {
    fn from(val: StrongIndex<T>) -> Self {
        val.demote()
    }
}