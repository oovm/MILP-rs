use crate::MixedValue;
use num::ToPrimitive;
use std::{
    cmp::Ordering,
    ops::{Add, AddAssign},
};

impl PartialEq<Self> for MixedValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (MixedValue::Decimal(lhs), MixedValue::Decimal(rhs)) => lhs.eq(rhs),
            (MixedValue::Integer(lhs), MixedValue::Integer(rhs)) => lhs.eq(rhs),
            (MixedValue::Decimal(lhs), MixedValue::Integer(rhs)) | (MixedValue::Integer(rhs), MixedValue::Decimal(lhs)) => {
                match rhs.to_f64() {
                    None => false,
                    Some(s) => lhs.eq(&s),
                }
            }

            (MixedValue::Boolean(b1), MixedValue::Boolean(b2)) => b1.eq(b2),
            (MixedValue::Decimal(_), MixedValue::Boolean(_)) => todo!(),
            (MixedValue::Boolean(_), MixedValue::Decimal(_)) => todo!(),
            (MixedValue::Integer(_), MixedValue::Boolean(_)) => todo!(),
            (MixedValue::Boolean(_), MixedValue::Integer(_)) => todo!(),
        }
    }
}

impl PartialOrd for MixedValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (MixedValue::Boolean(b1), MixedValue::Boolean(b2)) => b1.partial_cmp(b2),
            (MixedValue::Boolean(_), _) | (_, MixedValue::Boolean(_)) => None,
            (MixedValue::Decimal(d1), MixedValue::Decimal(d2)) => d1.partial_cmp(d2),
            (MixedValue::Integer(i1), MixedValue::Integer(i2)) => i1.partial_cmp(i2),
            (MixedValue::Decimal(_), MixedValue::Integer(_)) => todo!(),
            (MixedValue::Integer(_), MixedValue::Decimal(_)) => todo!(),
        }
    }
}

impl AddAssign<Self> for MixedValue {
    fn add_assign(&mut self, rhs: Self) {
        *self = rhs.add(self);
    }
}

impl Add<&MixedValue> for MixedValue {
    type Output = Self;

    fn add(self, rhs: &MixedValue) -> Self::Output {
        match (self, rhs) {
            (MixedValue::Decimal(lhs), MixedValue::Decimal(rhs)) => MixedValue::Decimal(lhs + rhs),
            (MixedValue::Integer(lhs), MixedValue::Integer(rhs)) => MixedValue::Integer(lhs + rhs),
            (MixedValue::Decimal(lhs), MixedValue::Integer(rhs)) => match rhs.to_f64() {
                Some(s) => MixedValue::Decimal(lhs + s),
                None => todo!(),
            },
            (MixedValue::Integer(lhs), MixedValue::Decimal(rhs)) => match lhs.to_f64() {
                Some(s) => MixedValue::Decimal(s + rhs),
                None => todo!(),
            },
            (MixedValue::Boolean(_), _) | (_, MixedValue::Boolean(_)) => todo!(),
        }
    }
}
