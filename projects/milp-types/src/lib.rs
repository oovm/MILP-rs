mod errors;
mod traits;

mod constraints;
mod equations;
mod vars;
pub use crate::{
    constraints::{LinearConstraint, OptimizeDirection},
    equations::{LinearCoefficient, LinearEquation, MixedEquation},
    errors::{LpError, LpErrorKind, LpResult},
    traits::LinearSolver,
    vars::{LinearVariable, MixedVariable},
};
pub use num::BigInt;
pub mod mixed;
pub mod utils;
