use ggez::GameError;
use std::io;
#[warn(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum RLError {
    UiError(GameError),
    DeserializeError(serde_yaml::Error),
    IOError(io::Error),
}

impl From<GameError> for RLError {
    fn from(e: GameError) -> Self {
        RLError::UiError(e)
    }
}

impl From<serde_yaml::Error> for RLError {
    fn from(e: serde_yaml::Error) -> Self {
        RLError::DeserializeError(e)
    }
}

impl From<std::io::Error> for RLError {
    fn from(e: std::io::Error) -> Self {
        RLError::IOError(e)
    }
}
