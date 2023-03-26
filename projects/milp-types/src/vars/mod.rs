use num::BigInt;

pub type FloatLinearVariable = LinearVariable<f64>;

pub type IntegerLinearVariable = LinearVariable<BigInt>;


pub struct LinearVariable<T> {
    symbol: String,
    upper: Option<(T, bool)>,
    lower: Option<(T, bool)>,
}



/// Create a new variable with constraints
impl<T> LinearVariable<T> {
    /// Creates a new variable with no constraints.
    pub fn new<S>(symbol: S) -> Self where S: Into<String> {
        Self {
            symbol: symbol.into(),
            upper: None,
            lower: None,
        }
    }
    /// Creates a new variable with the given lower bound.
    pub fn new_ge<S>(symbol: S, lower: T) -> Self where S: Into<String> {
        Self {
            symbol: symbol.into(),
            upper: None,
            lower: Some((lower, false)),
        }
    }
    /// Creates a new variable with the given lower bound.
    pub fn new_geq<S>(symbol: S, lower: T) -> Self where S: Into<String> {
        Self {
            symbol: symbol.into(),
            upper: None,
            lower: Some((lower, true)),
        }
    }
    /// Creates a new variable with the given upper bound.
    pub fn new_le<S>(symbol: S, upper: T) -> Self where S: Into<String> {
        Self {
            symbol: symbol.into(),
            upper: Some((upper, false)),
            lower: None,
        }
    }
    /// Creates a new variable with the given upper bound.
    pub fn new_leq<S>(symbol: S, upper: T) -> Self where S: Into<String> {
        Self {
            symbol: symbol.into(),
            upper: Some((upper, true)),
            lower: None,
        }
    }
    /// Creates a new variable with the given lower and upper bounds.
    pub fn new_between<S>(symbol: S, lower: T, upper: T) -> Self where S: Into<String> {
        Self {
            symbol: symbol.into(),
            upper: Some((upper, true)),
            lower: Some((lower, true)),
        }
    }
    /// Creates a new variable with the given lower and upper bounds.
    pub fn new_bounds<S>(symbol: S, lower: Option<T>, lower_inclusive: bool, upper: Option<T>, upper_inclusive: bool) -> Self where S: Into<String> {
        Self {
            symbol: symbol.into(),
            upper: upper.map(|upper| (upper, upper_inclusive)),
            lower: lower.map(|lower| (lower, lower_inclusive)),
        }
    }
}

impl<T> LinearVariable<T> {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }
}

impl<T: PartialOrd> LinearVariable<T> {
    pub fn contains(&self, other: &T) -> bool {
        self.satisfy_lower(other) && self.satisfy_upper(other)
    }

    pub fn satisfy_lower(&self, other: &T) -> bool {
        match self.lower {
            Some((ref lower, inclusive)) => {
                if inclusive {
                    other >= lower
                } else {
                    other > lower
                }
            }
            None => true,
        }
    }

    pub fn satisfy_upper(&self, other: &T) -> bool {
        match self.upper {
            Some((ref upper, inclusive)) => {
                if inclusive {
                    other <= upper
                } else {
                    other < upper
                }
            }
            None => true,
        }
    }
}