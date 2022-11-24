use crate::backend::screen::StackCommand;
use ggez::GameError;
use std::io;
use std::sync::mpsc::SendError;
use tracing::error;

#[warn(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum RLError {
    Ui(GameError),
    AssetError(String),
    Deserialization(serde_yaml::Error),
    IO(io::Error),
}

impl From<GameError> for RLError {
    fn from(e: GameError) -> Self {
        error!("GameError: {}", e);
        RLError::Ui(e)
    }
}

impl From<serde_yaml::Error> for RLError {
    fn from(e: serde_yaml::Error) -> Self {
        error!("Deserialization Error: {}", e);
        RLError::Deserialization(e)
    }
}

impl From<io::Error> for RLError {
    fn from(e: io::Error) -> Self {
        error!("IO Error: {}", e);
        RLError::IO(e)
    }
}

impl From<SendError<StackCommand>> for RLError {
    fn from(value: SendError<StackCommand>) -> Self {
        error!("Could not send StackCommand: {}", value);
        RLError::IO(io::Error::new(
            io::ErrorKind::Other,
            format!("Could not send StackCommand: {}", value),
        ))
    }
}
