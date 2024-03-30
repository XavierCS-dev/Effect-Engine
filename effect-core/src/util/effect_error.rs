use std::fmt;

#[derive(Debug, Clone)]
pub struct EffectError {
    pub msg: String,
}

impl fmt::Display for EffectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Effect 2: {}", self.msg)
    }
}

impl std::error::Error for EffectError {}

impl EffectError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: String::from(msg),
        }
    }
}
