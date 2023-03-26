use super::*;

impl Debug for MixedValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MixedValue::Decimal(d) => f.debug_tuple("Decimal").field(d).finish(),
            MixedValue::Integer(i) => f.debug_tuple("Integer").field(i).finish(),
            MixedValue::Boolean(b) => f.debug_tuple("Boolean").field(b).finish(),
        }
    }
}

impl Display for MixedValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MixedValue::Decimal(d) => write!(f, "{}", d),
            MixedValue::Integer(i) => write!(f, "{}", i),
            MixedValue::Boolean(b) => write!(f, "{}", b),
        }
    }
}
impl Debug for MixedVariable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.wrapper, f)
    }
}

impl Display for MixedVariable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.wrapper, f)
    }
}

impl Debug for MixedEquation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.wrapper, f)
    }
}
impl Display for MixedEquation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.wrapper, f)
    }
}
