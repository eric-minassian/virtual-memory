use crate::error::VMResult;

use super::data::{
    frame_offset::{self, FrameOffset},
    segment_offset::{self, SegmentOffset},
    segment_size::{self, SegmentSize},
};

#[derive(Debug, PartialEq, Eq)]
pub struct STInput {
    pub s: segment_offset::Value,
    pub z: segment_size::Value,
    pub f: frame_offset::Value,
}

impl STInput {
    /// Creates a new `STInput`.
    ///
    /// # Errors
    ///
    /// Returns an error if the segment, segment size, or frame are invalid.
    pub fn new(s: &str, z: &str, f: &str) -> VMResult<Self> {
        Ok(Self {
            s: SegmentOffset::new(s)?.value(),
            z: SegmentSize::new(z)?.value(),
            f: FrameOffset::new(f)?.value(),
        })
    }
}
