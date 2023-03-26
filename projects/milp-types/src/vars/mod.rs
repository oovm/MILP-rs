use std::cmp::Ordering;

pub struct LinearVariable<T> {
    symbol: String,
    upper: Option<(T, bool)>,
    lower: Option<(T, bool)>,
}

impl<T> LinearVariable<T> {
    pub fn new<S>(symbol: S) -> Self where S: Into<String> {
        Self {
            symbol: symbol.to_string(),
            upper: None,
            lower: None,
        }
    }
    pub fn new_ge<S>(symbol: S, constraint: T) -> Self {
        Self {
            symbol,
            upper: None,
            lower: Some((constraint, false)),
        }
    }
    pub fn new_geq<S>(symbol: S, constraint: T) -> Self {
        Self {
            symbol,
            upper: None,
            lower: Some((constraint, true)),
        }
    }
    pub fn new_le<S>(symbol: S, constraint: T) -> Self {
        Self {
            symbol,
            upper: Some((constraint, false)),
            lower: None,
        }
    }
    pub fn new_leq<S>(symbol: S, constraint: T) -> Self {
        Self {
            symbol,
            upper: Some((constraint, true)),
            lower: None,
        }
    }
    pub fn new_between<S>(symbol: S, lower: T, upper: T) -> Self {
        Self {
            symbol,
            upper: Some((upper, true)),
            lower: Some((lower, true)),
        }
    }
    pub fn new_bounds<S>(symbol: S, lower: Option<T>, lower_inclusive: bool, upper: T, upper_inclusive: bool) -> Self {
        Self {
            symbol,
            upper: Some((upper, false)),
            lower: Some((lower, false)),
        }
    }

}

