mod errors;
mod traits;

pub use crate::errors::{LpError, LpErrorKind, LpResult};

mod constraints;
mod equations;
mod vars;

pub use crate::equations::LinearEquation;

pub use num::BigInt;

pub use crate::{constraints::LinearConstraint, traits::LinearSolver, vars::LinearVariable};
