use ggez::GameError;
use std::io;
#[warn(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum RLError {
    Ui(GameError),
    Deserialization(serde_yaml::Error),
    IO(io::Error),
}

impl From<GameError> for RLError {
    fn from(e: GameError) -> Self {
        RLError::Ui(e)
    }
}

impl From<serde_yaml::Error> for RLError {
    fn from(e: serde_yaml::Error) -> Self {
        RLError::Deserialization(e)
    }
}

impl From<std::io::Error> for RLError {
    fn from(e: std::io::Error) -> Self {
        RLError::IO(e)
    }
}
