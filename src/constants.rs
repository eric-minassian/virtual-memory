pub const SEGMENT_SIZE_BITS: usize = 9;
pub const PAGE_SIZE_BITS: usize = 9;
pub const FRAME_SIZE_BITS: usize = 9;

pub const PAGE_SIZE: usize = 512;
pub const PAGE_COUNT: usize = 1024;
pub const SEGMENT_WORD_COUNT: usize = 2;
pub const SEGMENT_SIZE_OFFSET: usize = 0;
pub const SEGMENT_PAGE_TABLE_OFFSET: usize = 1;

pub const MAX_SEGMENT_OFFSET: u16 = (1 << SEGMENT_SIZE_BITS) - 1;
pub const MAX_SEGMENT_SIZE: u32 =
    (PAGE_SIZE * PAGE_COUNT - ((1 << SEGMENT_SIZE_BITS) * SEGMENT_WORD_COUNT)) as u32;
pub const MIN_POSITIVE_PAGE_OFFSET: i16 =
    (((1 << SEGMENT_SIZE_BITS) * SEGMENT_WORD_COUNT) / PAGE_SIZE) as i16;
pub const MAX_PAGE_OFFSET: i16 = PAGE_COUNT as i16 - 1;
