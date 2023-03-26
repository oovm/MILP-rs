use crate::{LinearConstraint, LpResult};
use num_traits::Zero;
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
    mem::take,
    ops::{AddAssign, Neg},
};

mod arithmetic;
mod display;

#[derive(Debug)]
pub struct LinearEquation<T> {
    lhs: LinearExpression<T>,
    ops: LinearEquationRelation,
    rhs: LinearExpression<T>,
}

#[derive(Debug)]
pub struct LinearExpression<T> {
    constant: T,
    coefficients: BTreeMap<String, LinearCoefficient<T>>,
}

#[derive(Debug)]
pub enum LinearEquationRelation {
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinearCoefficient<T> {
    symbol: String,
    coefficients: T,
}

impl<T> LinearCoefficient<T> {}

impl<T> LinearEquation<T> {
    pub fn le(lhs: LinearExpression<T>) -> Self
    where
        T: Zero,
    {
        Self { lhs, ops: LinearEquationRelation::Less, rhs: LinearExpression::new(T::zero()) }
    }
}

impl<T> LinearEquation<T> {
    pub fn normalize(&mut self) -> Self
    where
        T: Default,
    {
        match self.ops.needs_flip() {
            true => {
                let lhs = take(&mut self.lhs);
                self.rhs -= lhs;
                self.flip()
            }
            false => {
                let rhs = take(&mut self.rhs);
                self.lhs -= rhs;
            }
        }
    }
    pub fn flip(&mut self) {
        self.ops.flip();
        std::mem::swap(&mut self.lhs, &mut self.rhs);
    }
}

impl LinearEquationRelation {
    pub fn needs_flip(&self) -> bool {
        match self {
            LinearEquationRelation::Equal => false,
            LinearEquationRelation::NotEqual => false,
            LinearEquationRelation::Greater => true,
            LinearEquationRelation::GreaterOrEqual => true,
            LinearEquationRelation::Less => false,
            LinearEquationRelation::LessOrEqual => false,
        }
    }
    pub fn flip(&mut self) {
        *self = self.neg();
    }
}

impl Neg for LinearEquationRelation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            LinearEquationRelation::Equal => LinearEquationRelation::Equal,
            LinearEquationRelation::NotEqual => LinearEquationRelation::NotEqual,
            LinearEquationRelation::Greater => LinearEquationRelation::Less,
            LinearEquationRelation::GreaterOrEqual => LinearEquationRelation::LessOrEqual,
            LinearEquationRelation::Less => LinearEquationRelation::Greater,
            LinearEquationRelation::LessOrEqual => LinearEquationRelation::GreaterOrEqual,
        }
    }
}
