use crate::backend::gamestate::GameCommand;
use crate::backend::screen::StackCommand;
use good_web_game::GameError;
use std::fmt::Display;
use std::io;
use std::sync::mpsc::SendError;
use thiserror::Error;
use tracing::error;

/// All Red Life errors
#[warn(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum RLError {
    /// All Errors caused by Drawing
    Ui(GameError),
    /// Errors caused by loading the assets
    AssetError(String),
    /// Errors caused by loading the Gamestate from a file
    Deserialization(serde_yaml::Error),
    /// FileSystem and other errors
    IO(io::Error),
    /// Errors where senders/receivers were not intialized properly
    InitError(String),
}

impl Display for RLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RLError::Ui(e) => write!(f, "UI Error: {}", e),
            RLError::AssetError(e) => write!(f, "Asset Error: {}", e),
            RLError::Deserialization(e) => write!(f, "Deserialization Error: {}", e),
            RLError::IO(e) => write!(f, "IO Error: {}", e),
            RLError::InitError(e) => write!(f, "Init Error: {}", e),
        }
    }
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
/// Creates an Io Error
fn create_io_error(message: &str, value: impl std::fmt::Display) -> RLError {
    error!("{}: {}", message, value);
    RLError::IO(io::Error::new(
        io::ErrorKind::Other,
        format!("{message}: {value}"),
    ))
}
/// Macro for converting a `SendError` to an `RLError`
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
