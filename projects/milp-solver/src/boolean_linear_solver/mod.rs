use std::collections::BTreeSet;

pub struct BoolLinearSolver {
    variables: BTreeSet<String>,
    constraints: Vec<String>,
    epsilon: f64,
}
