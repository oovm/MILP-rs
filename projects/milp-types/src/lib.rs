mod errors;
mod traits;

use std::collections::BTreeSet;
pub use errors::{Error, Result};
mod vars;

pub use num::BigInt;

pub use crate::traits::LinearSolver;