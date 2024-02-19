use crate::{
    constants::MAX_SEGMENT_OFFSET,
    error::{VMError, VMResult},
};

pub type Value = u16;

#[derive(Debug, PartialEq, Eq)]
pub struct SegmentOffset(Value);

impl SegmentOffset {
    pub fn new(input: &str) -> VMResult<Self> {
        let input = input.parse().map_err(|_| VMError::InvalidSegment)?;

        if input > MAX_SEGMENT_OFFSET {
            return Err(VMError::InvalidSegment);
        }

        Ok(Self(input))
    }

    pub const fn value(&self) -> Value {
        self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let result = SegmentOffset::new("0");
        assert_eq!(result, Ok(SegmentOffset(0)));
    }

    #[test]
    fn negative() {
        let result = SegmentOffset::new("-1");
        assert_eq!(result, Err(VMError::InvalidSegment));
    }

    #[test]
    fn too_large() {
        let result = SegmentOffset::new("512");
        assert_eq!(result, Err(VMError::InvalidSegment));
    }

    #[test]
    fn max() {
        let result = SegmentOffset::new("511");
        assert_eq!(result, Ok(SegmentOffset(511)));
    }
}
