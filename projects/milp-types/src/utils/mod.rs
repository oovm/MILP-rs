use std::fmt::{Debug, Display, Formatter};

pub struct DisplayWrapper<'i, T> {
    inner: &'i T,
}

impl<T: Display> Debug for DisplayWrapper<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl<'i, T> DisplayWrapper<'i, T> {
    pub fn new(inner: &'i T) -> Self {
        Self { inner }
    }
}

pub struct DisplayVector<'i, T> {
    inner: &'i [T],
}

impl<T: Display> Debug for DisplayVector<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.inner.iter().map(|x| DisplayWrapper::new(x))).finish()
    }
}

impl<'i, T> DisplayVector<'i, T> {
    pub fn new(inner: &'i [T]) -> Self {
        Self { inner }
    }
}
