use ggez::GameError;
use std::io;
#[warn(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum RedError {
    UiError(GameError),
    DeserializeError(serde_yaml::Error),
    IOError(io::Error),
}

impl From<GameError> for RedError {
    fn from(e: GameError) -> Self {
        RedError::UiError(e)
    }
}

impl From<serde_yaml::Error> for RedError {
    fn from(e: serde_yaml::Error) -> Self {
        RedError::DeserializeError(e)
    }
}

impl From<std::io::Error> for RedError {
    fn from(e: std::io::Error) -> Self {
        RedError::IOError(e)
    }
}
