#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum VMError {
    GeneralError,
    MemoryFull,
}

impl From<std::num::TryFromIntError> for VMError {
    fn from(_: std::num::TryFromIntError) -> Self {
        Self::GeneralError
    }
}

impl From<std::convert::Infallible> for VMError {
    fn from(_: std::convert::Infallible) -> Self {
        Self::GeneralError
    }
}

pub type VMResult<T> = Result<T, VMError>;
