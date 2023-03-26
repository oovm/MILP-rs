mod errors;

pub use errors::{Error, Result};

mod float_linear_solver;

pub struct FloatLinearSolver {
    variables: BTreeSet<String>,
    constraints: Vec<String>,
}