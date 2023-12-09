use windows::Win32::Foundation::{D2DERR_RECREATE_TARGET, E_NOINTERFACE};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("need to recreate the target")]
    NeedRecreateTarget,
    #[error("No interface")]
    NoInterface,
    #[error(transparent)]
    Api(windows::core::Error),
}

impl From<windows::core::Error> for Error {
    #[inline]
    fn from(value: windows::core::Error) -> Self {
        match value.code() {
            D2DERR_RECREATE_TARGET => Self::NeedRecreateTarget,
            E_NOINTERFACE => Self::NoInterface,
            _ => Self::Api(value),
        }
    }
}

pub type Result<T> = ::core::result::Result<T, Error>;
