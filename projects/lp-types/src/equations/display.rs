use super::*;
use crate::utils::{DisplayMap, DisplayWrapper};

impl<T: Display> Display for LinearEquation<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cs: DisplayMap<&str, &T> = self.get_coefficients().collect();
        f.debug_struct("Equation").field("coefficients", &cs).field("constraint", &DisplayWrapper::new(&self.ops)).finish()
    }
}

impl<T: Display> Display for LinearCoefficient<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} × {}", self.coefficients, self.symbol)
    }
}
