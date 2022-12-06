use crate::backend::gamestate::GameCommand;
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
    InitError(String),
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

fn create_io_error(message: &str, value: impl std::fmt::Display) -> RLError {
    error!("{}: {}", message, value);
    RLError::IO(io::Error::new(
        io::ErrorKind::Other,
        format!("{}: {}", message, value),
    ))
}
/// Macro for converting a SendError to an RLError
#[macro_export]
macro_rules! convert_senderror {
    ($($command:ty),*) => {
        $(
            impl From<SendError<$command>> for RLError {
                fn from(value: SendError<$command>) -> Self {
                    create_io_error(stringify!($command), value)
                }
            }
        )*
    }
}
convert_senderror!(GameCommand, StackCommand);
