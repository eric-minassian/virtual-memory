use crate::{
    error::VMResult,
    io::data::{frame_offset::FrameOffset, page_offset::PageOffset, segment_offset::SegmentOffset},
};

use super::data::{frame_offset, page_offset, segment_offset};

#[derive(Debug, PartialEq, Eq)]
pub struct PTInput {
    pub segment: segment_offset::Value,
    pub page: page_offset::Value,
    pub frame: frame_offset::Value,
}

impl PTInput {
    /// Creates a new `PTInput`.
    ///
    /// # Errors
    ///
    /// Returns an error if the segment, page, or frame are invalid.
    pub fn new(segment: &str, page: &str, frame: &str) -> VMResult<Self> {
        Ok(Self {
            segment: SegmentOffset::new(segment)?.value(),
            page: PageOffset::new(page)?.value(),
            frame: FrameOffset::new(frame)?.value(),
        })
    }
}
