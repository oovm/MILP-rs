use std::collections::BTreeSet;

pub struct MixedLinearSolver {
    variables: BTreeSet<String>,
    constraints: Vec<String>,
    epsilon: f64,
}