use ggez::GameError;

#[derive(Debug)]
pub enum RedError {
    UiError,
    DeserializeError,
    IOError,
}

impl From<GameError> for RedError {
    fn from(_error: GameError) -> Self {
        RedError::UiError
    }
}