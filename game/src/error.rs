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
