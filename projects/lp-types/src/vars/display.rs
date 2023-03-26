use super::*;

impl<T> Debug for LinearVariable<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Variable")
            .field("symbol", &DisplayWrapper::new(&self.symbol))
            .field("bound", &DisplayWrapper::new(&self.bound))
            .finish()
    }
}

impl<T> Display for LinearVariable<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Variable({}, {})", self.symbol, self.bound)
    }
}
