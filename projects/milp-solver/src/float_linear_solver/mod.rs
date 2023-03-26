use std::collections::BTreeSet;
use milp_types::LinearSolver;

/// A linear solver that uses floating point numbers.
///
/// All variables can be float.
pub struct FloatLinearSolver {
    variables: BTreeSet<String>,
    constraints: Vec<String>,
    /// The epsilon value used to determine if a number is zero.
    pub epsilon: f64,
}


impl LinearSolver for FloatLinearSolver {}

impl FloatLinearSolver {
    pub fn new() -> Self {
        Self {
            variables: BTreeSet::new(),
            constraints: Vec::new(),
            epsilon: 1e-6,
        }
    }
    /// Creates a new solver with the given epsilon value.
    pub fn with_epsilon(epsilon: f64) -> Self {
        Self {
            variables: BTreeSet::new(),
            constraints: Vec::new(),
            epsilon,
        }
    }
}