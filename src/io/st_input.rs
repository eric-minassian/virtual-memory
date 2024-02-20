use crate::error::VMResult;

use super::data::{
    frame_offset::{self, FrameOffset},
    segment_offset::{self, SegmentOffset},
    segment_size::{self, SegmentSize},
};

#[derive(Debug, PartialEq, Eq)]
pub struct STInput {
    pub segment: segment_offset::Value,
    pub size: segment_size::Value,
    pub frame: frame_offset::Value,
}

impl STInput {
    /// Creates a new `STInput`.
    ///
    /// # Errors
    ///
    /// Returns an error if the segment, segment size, or frame are invalid.
    pub fn new(segment: &str, size: &str, frame: &str) -> VMResult<Self> {
        Ok(Self {
            segment: SegmentOffset::new(segment)?.value(),
            size: SegmentSize::new(size)?.value(),
            frame: FrameOffset::new(frame)?.value(),
        })
    }
}
