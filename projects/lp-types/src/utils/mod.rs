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

pub struct DisplayList<'i, T> {
    inner: &'i [T],
}

impl<T: Display> Debug for DisplayList<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.inner.iter().map(|x| DisplayWrapper::new(x))).finish()
    }
}

impl<'i, T> DisplayList<'i, T> {
    pub fn new(inner: &'i [T]) -> Self {
        Self { inner }
    }
}

pub struct DisplayMap<K, V> {
    inner: Vec<(K, V)>,
}

impl<K, V> Debug for DisplayMap<K, V>
where
    K: Display,
    V: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.inner.iter().map(|(k, v)| (DisplayWrapper::new(k), DisplayWrapper::new(v)))).finish()
    }
}

impl<K, V> FromIterator<(K, V)> for DisplayMap<K, V> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (K, V)>,
    {
        Self { inner: iter.into_iter().collect() }
    }
}
