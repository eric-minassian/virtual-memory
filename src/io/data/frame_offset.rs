use crate::{
    constants::{MAX_PAGE_OFFSET, MIN_POSITIVE_PAGE_OFFSET},
    error::{VMError, VMResult},
};

pub type Value = i16;

#[derive(Debug, PartialEq, Eq)]
pub struct FrameOffset(Value);

impl FrameOffset {
    pub fn new(input: &str) -> VMResult<Self> {
        let input = input.parse().map_err(|_| VMError::InvalidFrame)?;

        if (0..MIN_POSITIVE_PAGE_OFFSET).contains(&input) || input > MAX_PAGE_OFFSET {
            return Err(VMError::InvalidFrame);
        }

        Ok(Self(input))
    }

    #[must_use]
    pub const fn value(&self) -> Value {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let result = FrameOffset::new("2");
        assert_eq!(result, Ok(FrameOffset(2)));
    }

    #[test]
    fn negative() {
        let result = FrameOffset::new("-1");
        assert_eq!(result, Ok(FrameOffset(-1)));
    }

    #[test]
    fn zero() {
        let result = FrameOffset::new("0");
        assert_eq!(result, Err(VMError::InvalidFrame));
    }

    #[test]
    fn one() {
        let result = FrameOffset::new("1");
        assert_eq!(result, Err(VMError::InvalidFrame));
    }

    #[test]
    fn too_large() {
        let result = FrameOffset::new("1024");
        assert_eq!(result, Err(VMError::InvalidFrame));
    }

    #[test]
    fn max() {
        let result = FrameOffset::new("1023");
        assert_eq!(result, Ok(FrameOffset(1023)));
    }
}
