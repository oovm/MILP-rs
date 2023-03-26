use std::collections::{BTreeMap};
use milp_types::{FloatLinearVariable, LinearSolver, LinearVariable};

/// A linear solver that uses floating point numbers.
///
/// All variables can be float.
pub struct FloatLinearSolver {
    variables: BTreeMap<String, LinearVariable<f64>>,
    constraints: Vec<String>,
    /// The epsilon value used to determine if a number is zero.
    epsilon: f64,
}


impl LinearSolver for FloatLinearSolver {}

impl FloatLinearSolver {
    pub fn new() -> Self {
        Self {
            variables: BTreeMap::new(),
            constraints: Vec::new(),
            epsilon: 1e-6,
        }
    }
    pub fn add_variable(&mut self, variable: FloatLinearVariable) {
        self.variables.insert(variable.symbol().to_string(), variable);
    }
    pub fn add_constraint(&mut self, symbol: String) {
        self.constraints.push(symbol);
    }

    /// Creates a new solver with the given epsilon value.
    pub fn set_epsilon(epsilon: f64) -> Self {
        Self {
            variables: BTreeMap::new(),
            constraints: Vec::new(),
            epsilon,
        }
    }
}

#[test]
fn test() {
    let var = FloatLinearVariable::new("x");
    println!("{}", var.contains(&0.0));
    println!("{}", var.contains(&1.0));
    println!("{}", var.contains(&-1.0));
}