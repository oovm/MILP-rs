#[derive(Debug, Copy, Clone)]
pub enum LpErrorKind {
    UnknownError,
}

pub struct LpError {}

pub type LpResult<T = ()> = std::result::Result<T, LpErrorKind>;
