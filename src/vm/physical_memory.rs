use crate::constants::{PAGE_COUNT, PAGE_SIZE};

use super::frame::Frame;

pub type PhysicalMemory = Vec<Frame>;

#[allow(clippy::module_name_repetitions)]
pub trait PhysicalMemoryTrait {
    fn new_memory() -> Self;
    fn get_word_by_address(&self, address: usize) -> i32;
    fn get_word_by_offset(&self, frame: usize, offset: usize) -> i32;
    fn set_word_by_address(&mut self, address: usize, value: i32);
    fn set_word_by_offset(&mut self, frame: usize, offset: usize, value: i32);
}

impl PhysicalMemoryTrait for PhysicalMemory {
    fn new_memory() -> Self {
        vec![Frame::new(); PAGE_COUNT]
    }

    fn get_word_by_address(&self, address: usize) -> i32 {
        self[address / PAGE_SIZE].data[address % PAGE_SIZE]
    }

    fn get_word_by_offset(&self, frame: usize, offset: usize) -> i32 {
        self[frame].data[offset]
    }

    fn set_word_by_address(&mut self, address: usize, value: i32) {
        self[address / PAGE_SIZE].data[address % PAGE_SIZE] = value;
    }

    fn set_word_by_offset(&mut self, frame: usize, offset: usize, value: i32) {
        self[frame].data[offset] = value;
    }
}
