use crate::{
    constants::MAX_PAGE_OFFSET,
    error::{VMError, VMResult},
};

pub type Value = u16;

#[derive(Debug, PartialEq, Eq)]
pub struct PageOffset(Value);

impl PageOffset {
    pub fn new(input: &str) -> VMResult<Self> {
        let input = input.parse().map_err(|_| VMError::InvalidPage)?;

        if input > MAX_PAGE_OFFSET as Value {
            return Err(VMError::InvalidPage);
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
        let result = PageOffset::new("0");
        assert_eq!(result, Ok(PageOffset(0)));
    }

    #[test]
    fn negative() {
        let result = PageOffset::new("-1");
        assert_eq!(result, Err(VMError::InvalidPage));
    }

    #[test]
    fn too_large() {
        let result = PageOffset::new("1024");
        assert_eq!(result, Err(VMError::InvalidPage));
    }

    #[test]
    fn max() {
        let result = PageOffset::new("1023");
        assert_eq!(result, Ok(PageOffset(1023)));
    }
}
