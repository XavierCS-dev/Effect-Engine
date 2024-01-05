use std::fmt;

#[derive(Debug, Clone)]
pub struct EffectError {
    pub msg: String,
}

type Result<T> = std::result::Result<T, EffectError>;

impl fmt::Display for EffectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for EffectError {}
