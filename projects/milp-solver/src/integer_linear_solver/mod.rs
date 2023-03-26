use std::collections::BTreeSet;

pub struct IntegerLinearSolver {
    variables: BTreeSet<String>,
    constraints: Vec<String>,
    epsilon: f64,
}