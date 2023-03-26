use std::collections::BTreeSet;

pub struct FloatLinearSolver {
    variables: BTreeSet<String>,
    constraints: Vec<String>,
    epsilon: f64,
}