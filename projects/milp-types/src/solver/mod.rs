use crate::{MixedEquation, MixedValue, MixedVariable};
use lp_types::{
    utils::{DisplayList, DisplayWrapper},
    LinearEquation, LinearSolver, LinearVariable, OptimizeDirection,
};
use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
};

mod display;

pub struct MixedLinearDescriptor {
    variables: BTreeMap<String, MixedVariable>,
    constraints: Vec<MixedEquation>,
    target: MixedEquation,
    epsilon: f64,
}

impl LinearSolver for MixedLinearDescriptor {}

impl MixedLinearDescriptor {
    pub fn new(object: MixedEquation) -> Self {
        Self { variables: BTreeMap::new(), constraints: Vec::new(), target: OptimizeDirection::Maximize, epsilon: 1e-6 }
    }
    pub fn minimize() -> Self {
        Self { variables: BTreeMap::new(), constraints: Vec::new(), target: OptimizeDirection::Minimize, epsilon: 1e-6 }
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

impl MixedLinearDescriptor {
    pub fn normalized(&self) -> MixedLinearDescriptor {
        let mut normed = MixedLinearDescriptor::minimize();
        match self.target {
            OptimizeDirection::Maximize => {}
            OptimizeDirection::Minimize => {}
        }
    }
}
