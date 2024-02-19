use crate::{
    error::VMResult,
    io::data::{frame_offset::FrameOffset, page_offset::PageOffset, segment_offset::SegmentOffset},
};

use super::data::{frame_offset, page_offset, segment_offset};

#[derive(Debug, PartialEq, Eq)]
pub struct PTInput {
    pub s: segment_offset::Value,
    pub p: page_offset::Value,
    pub f: frame_offset::Value,
}

impl PTInput {
    /// Creates a new `PTInput`.
    ///
    /// # Errors
    ///
    /// Returns an error if the segment, page, or frame are invalid.
    pub fn new(s: &str, p: &str, f: &str) -> VMResult<Self> {
        Ok(Self {
            s: SegmentOffset::new(s)?.value(),
            p: PageOffset::new(p)?.value(),
            f: FrameOffset::new(f)?.value(),
        })
    }
}
