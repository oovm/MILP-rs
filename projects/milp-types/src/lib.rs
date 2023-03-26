mod errors;
mod traits;


pub use errors::{Error, Result};

mod vars;
mod constraints;

pub use num::BigInt;

pub use crate::traits::LinearSolver;
pub use crate::vars::{FloatLinearVariable, IntegerLinearVariable, LinearVariable};
