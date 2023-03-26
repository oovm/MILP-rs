

pub enum LinearConstraint<T> {
    None,
    Greater {
        lower: T,
        inclusive: bool,
    },
    Less {
        upper: T,
        inclusive: bool,
    },
    Equal {
        value: T,
    },
    NotEqual {
        value: T,
    },
    Or {
        left: Box<LinearConstraint<T>>,
        right: Box<LinearConstraint<T>>,
    },
    And {
        left: Box<LinearConstraint<T>>,
        right: Box<LinearConstraint<T>>,
    },
}

impl<T> LinearConstraint<T> {
    pub fn contains(&self, value: &T) -> bool where T: PartialOrd {
        match self {
            LinearConstraint::None => true,
            LinearConstraint::Greater { lower, inclusive } => {
                if *inclusive {
                    value >= lower
                } else {
                    value > lower
                }
            }
            LinearConstraint::Less { upper, inclusive } => {
                if *inclusive {
                    value <= upper
                } else {
                    value < upper
                }
            }
            LinearConstraint::Equal { value: v } => value == v,
            LinearConstraint::NotEqual { value: v } => value != v,
            LinearConstraint::Or { left, right } => left.contains(value) || right.contains(value),
            LinearConstraint::And { left, right } => left.contains(value) && right.contains(value),
        }
    }
}