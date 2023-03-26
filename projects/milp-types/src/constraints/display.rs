use super::*;
use std::fmt::{Formatter, Write};

impl<T: Display> Display for LinearConstraint<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LinearConstraint::None => f.write_char('∀'),
            LinearConstraint::Greater { lower, inclusive } => {}
            LinearConstraint::Less { upper, inclusive } => {}
            LinearConstraint::Equal { value } => {}
            LinearConstraint::NotEqual {} => {}
            LinearConstraint::Or {} => {}
            LinearConstraint::And {} => {}
        }
    }
}
