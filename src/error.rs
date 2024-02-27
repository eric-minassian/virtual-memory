#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub enum VMError {
    MemoryFull,
    InvalidSegment,
    InvalidSegmentSize,
    InvalidFrame,
    InvalidPage,
    VirtualAddressLeadingBits,
    VirtualAddressOutOfBounds,
    MemoryNotInitialized,
    TryFromIntError,
    IOError(String),
}

impl From<std::num::TryFromIntError> for VMError {
    fn from(_: std::num::TryFromIntError) -> Self {
        Self::TryFromIntError
    }
}

impl From<std::convert::Infallible> for VMError {
    fn from(_: std::convert::Infallible) -> Self {
        Self::TryFromIntError
    }
}

impl From<std::io::Error> for VMError {
    fn from(error: std::io::Error) -> Self {
        Self::IOError(error.to_string())
    }
}

pub type VMResult<T> = Result<T, VMError>;
