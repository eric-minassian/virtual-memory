#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
pub enum VMError {
    GeneralError,
    MemoryFull,
    InvalidSegment,
    InvalidSegmentSize,
    InvalidFrame,
    InvalidPage,
    VirtualAddressLeadingBits,
    VirtualAddressOutOfBounds,
    MemoryNotInitialized,
    TryFromIntError,
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

pub type VMResult<T> = Result<T, VMError>;
