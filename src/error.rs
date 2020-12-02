use peach_lib::error::{NetworkError, OledError, StatsError};

#[derive(Debug)]
pub enum MenuError {
    NetworkError,
    OledError,
    StatsError,
}

impl From<NetworkError> for MenuError {
    fn from(_err: NetworkError) -> MenuError {
        MenuError::NetworkError
    }
}

impl From<OledError> for MenuError {
    fn from(_err: OledError) -> MenuError {
        MenuError::OledError
    }
}

impl From<StatsError> for MenuError {
    fn from(_err: StatsError) -> MenuError {
        MenuError::StatsError
    }
}
