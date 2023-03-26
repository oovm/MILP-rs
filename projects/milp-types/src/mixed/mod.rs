use crate::{LinearEquation, LinearVariable};
use num::BigInt;

mod convert;
mod display;

#[derive(Debug)]
pub enum MixedValue {
    Boolean(bool),
    Decimal(f64),
    Integer(BigInt),
}

pub struct MixedVariable {
    wrapper: LinearVariable<MixedValue>,
}
pub struct MixedEquation {
    wrapper: LinearEquation<MixedValue>,
}
