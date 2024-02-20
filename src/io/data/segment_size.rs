use crate::{
    constants::MAX_SEGMENT_SIZE,
    error::{VMError, VMResult},
};

pub type Value = u32;

#[derive(Debug, PartialEq, Eq)]
pub struct SegmentSize(Value);

impl SegmentSize {
    pub fn new(input: &str) -> VMResult<Self> {
        let input = input.parse().map_err(|_| VMError::InvalidSegmentSize)?;

        if input > MAX_SEGMENT_SIZE {
            return Err(VMError::InvalidSegmentSize);
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
        let result = SegmentSize::new("0");
        assert_eq!(result, Ok(SegmentSize(0)));
    }

    #[test]
    fn negative() {
        let result = SegmentSize::new("-1");
        assert_eq!(result, Err(VMError::InvalidSegmentSize));
    }

    #[test]
    fn too_large() {
        let result = SegmentSize::new("523265");
        assert_eq!(result, Err(VMError::InvalidSegmentSize));
    }

    #[test]
    fn max() {
        let result = SegmentSize::new("523264");
        assert_eq!(result, Ok(SegmentSize(523_264)));
    }
}
