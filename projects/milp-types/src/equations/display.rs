use super::*;
use crate::utils::{DisplayVector, DisplayWrapper};

impl<T: Display> Display for LinearEquation<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let coefficients = self.coefficients.values().collect::<Vec<_>>();
        f.debug_struct("LinearEquation")
            .field("coefficients", &DisplayVector::new(&coefficients))
            .field("constraint", &DisplayWrapper::new(&self.constraint))
            .finish()
    }
}

impl<T: Display> Display for LinearCoefficient<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} × {}", self.coefficients, self.symbol)
    }
}
