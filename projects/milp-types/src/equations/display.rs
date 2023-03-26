use super::*;
use crate::utils::{DisplayList, DisplayMap, DisplayWrapper};

impl<T: Display> Display for LinearEquation<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cs: DisplayMap<&str, &T> = self.get_coefficients().collect();
        f.debug_struct("Equation")
            .field("coefficients", &cs)
            .field("constraint", &DisplayWrapper::new(&self.constraint))
            .finish()
    }
}

impl<T: Display> Display for LinearCoefficient<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} × {}", self.coefficients, self.symbol)
    }
}
