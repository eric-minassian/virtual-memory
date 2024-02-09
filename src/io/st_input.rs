use crate::error::{VMError, VMResult};

#[derive(Debug, PartialEq, Eq)]
pub struct STInput {
    pub s: u16,
    pub z: u32,
    pub f: i16,
}

impl STInput {
    /// Creates a new `STInput`.
    ///
    /// # Errors
    ///
    /// Returns an error if the segment, segment size, or frame are invalid.
    pub fn new(s: i32, z: i32, f: i32) -> VMResult<Self> {
        let s = u16::try_from(s).map_err(|_| VMError::InvalidSegment)?;
        let z = u32::try_from(z).map_err(|_| VMError::InvalidSegmentSize)?;
        let f = i16::try_from(f).map_err(|_| VMError::InvalidFrame)?;

        // if s > MAX_SEGMENT {
        //     return Err(VMError::InvalidSegment);
        // }

        // if z > MAX_SEGMENT_SIZE {
        //     return Err(VMError::InvalidSegmentSize);
        // }

        // if f >= 0 && f < MIN_POSITIVE_FRAME {
        //     return Err(VMError::InvalidFrame);
        // }

        // if f > MAX_FRAME {
        //     return Err(VMError::InvalidFrame);
        // }

        Ok(Self { s, z, f })
    }
}

// #[cfg(test)]
// mod tests {

//     use crate::constants::SEGMENT_SIZE_BITS;

//     use super::*;

//     #[test]
//     fn new_st_input() {
//         let st_input = STInput::new(0, 0, 2);
//         assert_eq!(st_input, Ok(STInput { s: 0, z: 0, f: 2 }));
//     }

//     #[test]
//     fn new_st_input_invalid_segment() {
//         let st_input = STInput::new(-1, 0, 2);
//         assert_eq!(st_input, Err(VMError::InvalidSegment));

//         let st_input = STInput::new(1 << SEGMENT_SIZE_BITS, 0, 2);
//         assert_eq!(st_input, Err(VMError::InvalidSegment));
//     }

//     #[test]
//     fn new_st_input_invalid_segment_size() {
//         let st_input = STInput::new(0, -1, 2);
//         assert_eq!(st_input, Err(VMError::InvalidSegmentSize));

//         let st_input = STInput::new(0, MAX_SEGMENT_SIZE as i32 + 1, 2);
//         assert_eq!(st_input, Err(VMError::InvalidSegmentSize));
//     }

//     #[test]
//     fn new_st_input_invalid_frame() {
//         let st_input = STInput::new(0, 0, -1).expect("Invalid frame");
//         assert_eq!(st_input, STInput { s: 0, z: 0, f: -1 });

//         let st_input = STInput::new(0, 0, MIN_POSITIVE_FRAME as i32 - 1);
//         assert_eq!(st_input, Err(VMError::InvalidFrame));

//         let st_input = STInput::new(0, 0, MAX_FRAME as i32 + 1);
//         assert_eq!(st_input, Err(VMError::InvalidFrame));
//     }
// }
