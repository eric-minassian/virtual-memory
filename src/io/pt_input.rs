use crate::error::{VMError, VMResult};

#[derive(Debug, PartialEq, Eq)]
pub struct PTInput {
    pub s: u16,
    pub p: u16,
    pub f: i16,
}

impl PTInput {
    /// Creates a new `PTInput`.
    ///
    /// # Errors
    ///
    /// Returns an error if the segment, page, or frame are invalid.
    pub fn new(s: i32, p: i32, f: i32) -> VMResult<Self> {
        let s = u16::try_from(s).map_err(|_| VMError::InvalidSegment)?;
        let p = u16::try_from(p).map_err(|_| VMError::InvalidPage)?;
        let f = i16::try_from(f).map_err(|_| VMError::InvalidFrame)?;

        // if s > MAX_SEGMENT {
        //     return Err(VMError::InvalidSegment);
        // }

        // if p > MAX_PAGE {
        //     return Err(VMError::InvalidPage);
        // }

        // if f >= 0 && f < MIN_POSITIVE_FRAME {
        //     return Err(VMError::InvalidFrame);
        // }

        // if f > MAX_FRAME {
        //     return Err(VMError::InvalidFrame);
        // }

        Ok(Self { s, p, f })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn new_pt_input() {
//         let pt_input = PTInput::new(0, 0, 2);
//         assert_eq!(pt_input, Ok(PTInput { s: 0, p: 0, f: 2 }));
//     }

//     #[test]
//     fn new_pt_input_invalid_segment() {
//         let pt_input = PTInput::new(-1, 0, 2);
//         assert_eq!(pt_input, Err(VMError::InvalidSegment));

//         let pt_input = PTInput::new(1 << 9, 0, 2);
//         assert_eq!(pt_input, Err(VMError::InvalidSegment));
//     }

//     #[test]
//     fn new_pt_input_invalid_page() {
//         let pt_input = PTInput::new(0, -1, 2);
//         assert_eq!(pt_input, Err(VMError::InvalidPage));

//         let pt_input = PTInput::new(0, 1 << 9, 2);
//         assert_eq!(pt_input, Err(VMError::InvalidPage));
//     }

//     #[test]
//     fn new_pt_input_invalid_frame() {
//         let pt_input = PTInput::new(0, 0, -1);
//         assert_eq!(pt_input, Err(VMError::InvalidFrame));

//         let pt_input = PTInput::new(0, 0, 1 << 9);
//         assert_eq!(pt_input, Err(VMError::InvalidFrame));
//     }
// }
