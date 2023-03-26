pub type LpResult<T = ()> = Result<T, LpError>;

#[derive(Debug, Clone)]
pub enum LpErrorKind {
    InvalidConstraint { label: String, message: String },
}

pub struct LpError {
    kind: Box<LpErrorKind>,
}

impl LpError {
    pub fn invalid_constraint<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        Self { kind: Box::new(LpErrorKind::InvalidConstraint { label: "".to_string(), message: message.into() }) }
    }
    pub fn with_label<S>(mut self, label: S) -> Self
    where
        S: AsRef<str>,
    {
        self.kind.set_label(label);
        self
    }
}

impl LpErrorKind {
    pub fn set_label<S>(&mut self, label: S)
    where
        S: AsRef<str>,
    {
        match self {
            LpErrorKind::InvalidConstraint { label: l, .. } => *l = label.as_ref().to_string(),
        }
    }
}
