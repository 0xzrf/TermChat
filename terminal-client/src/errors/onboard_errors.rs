use std::fmt::Display;

#[derive(Debug)]
pub enum OnboardErrors {
    CreateErrors(&'static str),
    JoinErrors(&'static str),
    ServerError(&'static str),
    ReadError(&'static str),
}

impl Display for OnboardErrors {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            OnboardErrors::CreateErrors(err_str) => write!(f, "Create Error: {err_str}"),
            OnboardErrors::JoinErrors(err_str) => write!(f, "Join Error: {err_str}"),
            OnboardErrors::ServerError(err_str) => write!(f, "Server Error: {err_str}"),
            OnboardErrors::ReadError(err_str) => write!(f, "Read Error: {err_str}"),
        }
    }
}

#[derive(Debug)]
pub enum CreateErrors {
    RoomNotCreated(&'static str),
}

#[derive(Debug)]
pub enum JoinErrors {
    RoomNotJoined(&'static str),
}

#[derive(Debug)]
pub enum ServerConnectionError {
    CouldntConnectServer(&'static str),
}

impl From<CreateErrors> for OnboardErrors {
    fn from(value: CreateErrors) -> Self {
        match value {
            CreateErrors::RoomNotCreated(str) => Self::CreateErrors(str),
        }
    }
}
impl From<ServerConnectionError> for OnboardErrors {
    fn from(value: ServerConnectionError) -> Self {
        match value {
            ServerConnectionError::CouldntConnectServer(str) => Self::CreateErrors(str),
        }
    }
}

impl From<JoinErrors> for OnboardErrors {
    fn from(value: JoinErrors) -> Self {
        match value {
            JoinErrors::RoomNotJoined(str) => Self::JoinErrors(str),
        }
    }
}
