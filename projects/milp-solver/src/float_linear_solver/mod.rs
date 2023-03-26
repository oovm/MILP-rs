use milp_types::{
    utils::{DisplayList, DisplayWrapper},
    LinearConstraint, LinearEquation, LinearSolver, LinearVariable, OptimizeDirection,
};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Display, Formatter},
};

mod display;
/// A linear solver that uses floating point numbers.
///
/// All variables can be float.

pub struct FloatLinearSolver {
    variables: BTreeMap<String, LinearVariable<f64>>,
    constraints: Vec<LinearEquation<f64>>,
    direct: OptimizeDirection,
    /// The epsilon value used to determine if a number is zero.
    epsilon: f64,
}

impl LinearSolver for FloatLinearSolver {}

impl FloatLinearSolver {
    pub fn new(maximize: bool) -> Self {
        Self { variables: BTreeMap::new(), constraints: Vec::new(), direct: OptimizeDirection::from(maximize), epsilon: 1e-6 }
    }
    pub fn get_variable(&self, symbol: &str) -> Option<&LinearVariable<f64>> {
        self.variables.get(symbol)
    }
    pub fn add_variable(&mut self, variable: LinearVariable<f64>) {
        self.variables.insert(variable.symbol().to_string(), variable);
    }
    pub fn add_equation(&mut self, equation: LinearEquation<f64>) {
        for variable in equation.variables() {
            if !self.variables.contains_key(variable) {
                self.variables.insert(variable.to_string(), LinearVariable::new(variable));
            }
        }
        self.constraints.push(equation);
    }
    /// Creates a new solver with the given epsilon value.
    pub fn set_epsilon(&mut self, epsilon: f64) {
        self.epsilon = epsilon;
    }
}

#[test]
fn test() {
    let mut problem = FloatLinearSolver::new(true);
    let mut e1 = LinearEquation::new(LinearConstraint::le(1.0)).unwrap();
    e1.add_coefficient(1.0, "x");
    e1.add_coefficient(1.0, "y");
    problem.add_equation(e1);

    let mut e2 = LinearEquation::new(LinearConstraint::le(2.0)).unwrap();
    e2.add_coefficient(1.0, "x");
    e2.add_coefficient(1.0, "z");
    problem.add_equation(e2);

    problem.add_variable(LinearVariable::new_le("x", 1.0));
    problem.add_variable(LinearVariable::new_le("y", 1.0));
    problem.add_variable(LinearVariable::new_le("z", 1.0));

    println!("{:#?}", problem);
}
