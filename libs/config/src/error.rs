use std::fmt::Display;

pub struct ConfigError(String);

pub type ConfigResult<T> = Result<T, ConfigError>;

pub(crate) fn new(msg: String) -> ConfigError {
    ConfigError(msg)
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
