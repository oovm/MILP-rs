mod errors;
mod traits;

mod constraints;
mod equations;
mod vars;
pub use crate::{
    constraints::{LinearConstraint, OptimizeDirection},
    equations::{LinearCoefficient, LinearEquation},
    errors::{LpError, LpErrorKind, LpResult},
    traits::LinearSolver,
    vars::LinearVariable,
};
pub use num::BigInt;
mod utils;
