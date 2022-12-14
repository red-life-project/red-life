use crate::backend::gamestate::GameCommand;
use crate::backend::screen::StackCommand;
use good_web_game::GameError;
use std::io;
use std::sync::mpsc::SendError;
use tracing::error;
use thiserror::Error;

/// All Red Life errors
#[warn(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum RLError {
    /// All Errors caused by Drawing
    #[error("Drawing Error: {0}")]
    Ui(#[from] GameError),
    /// Errors caused by loading the assets
    #[error("Error loading asset: {0}")]
    AssetError(String),
    /// Errors caused by loading the Gamestate from a file
    #[error("Error loading Gamestate: {0}")]
    Deserialization(#[from] serde_yaml::Error),
    /// FileSystem and other errors
    #[error("IO Error: {0}")]
    IO(#[from] io::Error),
    /// Errors where senders/receivers were not initialized properly
    #[error("Error with Sender/Receiver: {0}")]
    InitError(String),
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
