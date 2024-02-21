use crate::{
    constants::{
        PAGE_COUNT, PAGE_SIZE, SEGMENT_PAGE_TABLE_OFFSET, SEGMENT_SIZE_OFFSET, SEGMENT_WORD_COUNT,
    },
    error::{VMError, VMResult},
    io::{pt_input::PTInput, st_input::STInput},
    vm::{
        physical_memory::{PhysicalMemory, PhysicalMemoryTrait},
        virtual_address::VirtualAddress,
    },
};

pub type Address = u32;

#[derive(Debug, PartialEq, Eq)]
pub struct VirtualMemory {
    physical_memory: PhysicalMemory,
    disk: Vec<[i32; PAGE_SIZE]>,
}

impl VirtualMemory {
    pub fn new(segment_table_init: Vec<STInput>, page_table_init: Vec<PTInput>) -> VMResult<Self> {
        let mut physical_memory = PhysicalMemory::new_memory();
        let mut disk = vec![[0; PAGE_SIZE]; PAGE_COUNT];

        // Mark Segment Table Frames as taken
        physical_memory
            .iter_mut()
            .take(SEGMENT_WORD_COUNT)
            .for_each(|frame| frame.free = false);

        for st_input in segment_table_init {
            let segment_base_address = usize::from(st_input.segment) * SEGMENT_WORD_COUNT;

            physical_memory.set_word_by_address(
                segment_base_address + SEGMENT_SIZE_OFFSET,
                i32::try_from(st_input.size)?,
            );
            physical_memory.set_word_by_address(
                segment_base_address + SEGMENT_PAGE_TABLE_OFFSET,
                i32::try_from(st_input.frame)?,
            );

            // Mark Page Table Frames as taken
            if st_input.frame.is_positive() {
                physical_memory[usize::try_from(st_input.frame)?].free = false;
            }
        }

        for pt_input in page_table_init {
            let pt_frame_num = physical_memory.get_word_by_address(
                usize::from(pt_input.segment) * SEGMENT_WORD_COUNT + SEGMENT_PAGE_TABLE_OFFSET,
            );

            let page_offset = usize::from(pt_input.page);

            if pt_frame_num.is_negative() {
                disk[usize::try_from(pt_frame_num.abs())?][page_offset] = i32::from(pt_input.frame);
            } else {
                physical_memory.set_word_by_offset(
                    usize::try_from(pt_frame_num)?,
                    page_offset,
                    i32::from(pt_input.frame),
                );
            }

            // Mark Page Frames as taken
            if pt_input.frame.is_positive() {
                physical_memory[usize::try_from(pt_input.frame)?].free = false;
            }
        }

        Ok(Self {
            physical_memory,
            disk,
        })
    }

    fn allocate_page(&mut self) -> VMResult<usize> {
        self.physical_memory
            .iter_mut()
            .enumerate()
            .skip(SEGMENT_WORD_COUNT)
            .find(|(_, frame)| frame.free)
            .map(|(i, frame)| {
                frame.free = false;
                i
            })
            .ok_or(VMError::MemoryFull)
    }

    fn get_frame(&mut self, address: usize) -> VMResult<usize> {
        match self.physical_memory.get_word_by_address(address) {
            // Not Resident in Memory
            frame if frame < 0 => {
                let disk_frame = usize::try_from(frame.abs())?;
                let free_frame = self.allocate_page()?;

                self.physical_memory
                    .set_word_by_address(address, i32::try_from(free_frame)?);

                // Copy Frame From Disk to Memory
                for (i, &word) in self.disk[disk_frame].iter().enumerate() {
                    self.physical_memory.set_word_by_offset(free_frame, i, word);
                }

                Ok(free_frame)
            }
            // Not Initialized
            0 => Err(VMError::MemoryNotInitialized),

            // Resident in Memory
            offset => Ok(usize::try_from(offset)?),
        }
    }

    pub fn translate(&mut self, virtual_address: VirtualAddress) -> VMResult<Address> {
        let segment_base: usize = usize::from(virtual_address.s) * SEGMENT_WORD_COUNT;
        let segment_size = self
            .physical_memory
            .get_word_by_address(segment_base + SEGMENT_SIZE_OFFSET);

        if virtual_address.pw >= u32::try_from(segment_size)? {
            return Err(VMError::VirtualAddressOutOfBounds);
        }

        let page_table_frame = self.get_frame(segment_base + SEGMENT_PAGE_TABLE_OFFSET)?;
        let page_frame =
            self.get_frame(page_table_frame * PAGE_SIZE + usize::from(virtual_address.p))?;

        Ok(u32::try_from(
            page_frame * PAGE_SIZE + usize::from(virtual_address.w),
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn before() -> VirtualMemory {
        let st_inputs = vec![
            STInput::new("8", "4000", "3").expect("Failed to create PTInput"),
            STInput::new("9", "5000", "-7").expect("Failed to create PTInput"),
        ];
        let pt_inputs = vec![
            PTInput::new("8", "0", "10").expect("Failed to create PTInput"),
            PTInput::new("8", "1", "-20").expect("Failed to create PTInput"),
            PTInput::new("9", "0", "13").expect("Failed to create PTInput"),
            PTInput::new("9", "1", "-25").expect("Failed to create PTInput"),
        ];

        VirtualMemory::new(st_inputs, pt_inputs).expect("Failed to init")
    }

    #[test]
    fn init() {
        let virtual_memory = before();

        assert_eq!(virtual_memory.physical_memory[0].data[8 * 2], 4000);
        assert_eq!(virtual_memory.physical_memory[0].data[8 * 2 + 1], 3);
        assert_eq!(virtual_memory.physical_memory[0].data[9 * 2], 5000);
        assert_eq!(virtual_memory.physical_memory[0].data[9 * 2 + 1], -7);

        assert_eq!(virtual_memory.disk[7][0], 13);
        assert_eq!(virtual_memory.disk[7][1], -25);

        assert_eq!(virtual_memory.physical_memory[3].data[0], 10);
        assert_eq!(virtual_memory.physical_memory[3].data[1], -20);
    }

    #[test]
    fn simple_translate() {
        let mut vm = before();
        let virtual_address =
            VirtualAddress::new(2097162).expect("Failed to create VirtualAddress");
        let expected_address = 5130;

        let address = vm.translate(virtual_address).expect("Failed to translate");

        assert_eq!(address, expected_address);
    }

    #[test]
    fn pg_not_resident() {
        let mut vm = before();
        let virtual_address =
            VirtualAddress::new(2097674).expect("Failed to create VirtualAddress");
        let expected_address = 1034;

        let address = vm.translate(virtual_address).expect("Failed to translate");

        assert_eq!(address, expected_address);
    }

    #[test]
    fn pt_not_resident() {
        let mut vm = before();
        let virtual_address =
            VirtualAddress::new(2359306).expect("Failed to create VirtualAddress");
        let expected_address = 6666;

        let address = vm.translate(virtual_address).expect("Failed to translate");

        assert_eq!(address, expected_address);
    }

    #[test]
    fn pt_and_pg_not_resident() {
        let mut vm = before();
        let virtual_address =
            VirtualAddress::new(2359818).expect("Failed to create VirtualAddress");
        let expected_address = 2058;

        let address = vm.translate(virtual_address).expect("Failed to translate");

        assert_eq!(address, expected_address);
    }

    #[test]
    fn find_free_page() {
        let mut vm = VirtualMemory::new(vec![], vec![]).expect("Failed to init");

        let free_page = vm.allocate_page().expect("Failed to find free page");
        assert_eq!(free_page, 2);

        let free_page = vm.allocate_page().expect("Failed to find free page");
        assert_eq!(free_page, 3);
    }

    #[test]
    fn find_free_page_full() {
        let mut vm = VirtualMemory::new(vec![], vec![]).expect("Failed to init");

        for i in 0..PAGE_COUNT {
            vm.physical_memory[i].free = false;
        }

        let free_page = vm.allocate_page();
        assert_eq!(free_page.unwrap_err(), VMError::MemoryFull);
    }
}
