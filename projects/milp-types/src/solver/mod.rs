use crate::{MixedEquation, MixedVariable};
use lp_types::{
    utils::{DisplayList, DisplayWrapper},
    LinearSolver, OptimizeDirection,
};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
};

mod display;

pub struct MixedLinearDescriptor {
    variables: BTreeMap<String, MixedVariable>,
    constraints: Vec<MixedEquation>,
    direct: OptimizeDirection,
    epsilon: f64,
}

impl LinearSolver for MixedLinearDescriptor {}

impl MixedLinearDescriptor {
    pub fn new(maximize: bool) -> Self {
        Self { variables: BTreeMap::new(), constraints: Vec::new(), direct: OptimizeDirection::from(maximize), epsilon: 1e-6 }
    }
    pub fn get_variable(&self, symbol: &str) -> Option<&MixedVariable> {
        self.variables.get(symbol)
    }
    pub fn add_variable(&mut self, variable: MixedVariable) {
        self.variables.insert(variable.get_symbol().to_string(), variable);
    }
    pub fn add_equation(&mut self, equation: MixedEquation) {
        for variable in equation.variables() {
            if !self.variables.contains_key(variable) {
                self.variables.insert(variable.to_string(), MixedVariable::free(variable));
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
    let mut problem = MixedLinearDescriptor::new(true);
    let mut e1 = MixedEquation::new(MixedConstraint::le(1.0)).unwrap();
    e1.add_coefficient(1.0, "x");
    e1.add_coefficient(1.0, "y");
    problem.add_equation(e1);

    let mut e2 = LinearEquation::new(LinearConstraint::le(2.0)).unwrap();
    e2.add_coefficient(1.0, "x");
    e2.add_coefficient(1.0, "z");
    problem.add_equation(e2);

    problem.add_variable(LinearVariable::le("x", 1.0));
    problem.add_variable(LinearVariable::le("y", 1.0));
    problem.add_variable(LinearVariable::le("z", 1.0));

    println!("{:#?}", problem);
}
