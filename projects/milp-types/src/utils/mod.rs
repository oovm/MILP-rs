use std::fmt::{Debug, Display, Formatter};

pub struct DisplayInDebug<'i, T> {
    inner: &'i T,
}

impl<T: Display> Debug for DisplayInDebug<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl<'i, T> DisplayInDebug<'i, T> {
    pub fn new(inner: &'i T) -> Self {
        Self { inner }
    }
}
