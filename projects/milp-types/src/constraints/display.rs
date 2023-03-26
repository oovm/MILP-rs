use super::*;

impl<T: Display> Display for LinearConstraint<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LinearConstraint::None => f.write_char('∀'),
            LinearConstraint::Greater { lower, inclusive } => {
                if *inclusive {
                    write!(f, "≥ {}", lower)
                }
                else {
                    write!(f, "> {}", lower)
                }
            }
            LinearConstraint::Less { upper, inclusive } => {
                if *inclusive {
                    write!(f, "≤ {}", upper)
                }
                else {
                    write!(f, "< {}", upper)
                }
            }
            LinearConstraint::Equal { value } => {
                write!(f, "= {}", value)
            }
            LinearConstraint::NotEqual { value } => {
                write!(f, "≠ {}", value)
            }
            LinearConstraint::Or { left, right } => match should_add_parentheses(left, right) {
                true => write!(f, "({} ∨ {})", left, right),
                false => write!(f, "{} ∨ {}", left, right),
            },
            LinearConstraint::And { left, right } => match should_add_parentheses(left, right) {
                true => write!(f, "({} ∧ {})", left, right),
                false => write!(f, "{} ∧ {}", left, right),
            },
        }
    }
}

fn should_add_parentheses<T>(lhs: &LinearConstraint<T>, rhs: &LinearConstraint<T>) -> bool {
    match (lhs, rhs) {
        (LinearConstraint::Or { .. }, _) => true,
        (LinearConstraint::And { .. }, _) => true,
        _ => false,
    }
}
