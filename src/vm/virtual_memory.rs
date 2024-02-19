use crate::{
    constants::{
        PAGE_COUNT, PAGE_SIZE, SEGMENT_PAGE_TABLE_OFFSET, SEGMENT_SIZE_OFFSET, SEGMENT_WORD_COUNT,
    },
    error::{VMError, VMResult},
    io::{pt_input::PTInput, st_input::STInput},
    vm::virtual_address::VirtualAddress,
};

pub type Address = u32;

#[derive(Debug, PartialEq, Eq)]
pub struct VirtualMemory {
    physical_memory: Vec<i32>,
    disk: Vec<Vec<i32>>,
}

impl Default for VirtualMemory {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualMemory {
    #[must_use]
    pub fn new() -> Self {
        Self {
            physical_memory: vec![0; PAGE_SIZE * PAGE_COUNT],
            disk: vec![vec![0; PAGE_SIZE]; PAGE_COUNT],
        }
    }

    /// Initializes the virtual memory with the given segmentation and page table inputs.
    ///
    /// # Errors
    ///
    /// - `VMError::GeneralError` if an error occurs while converting the virtual address to an
    pub fn init(
        &mut self,
        segmentation_table_inputs: Vec<STInput>,
        page_table_inputs: Vec<PTInput>,
    ) -> VMResult<()> {
        for st_input in segmentation_table_inputs {
            let segment_address = usize::from(st_input.s) * SEGMENT_WORD_COUNT;

            self.physical_memory[segment_address + SEGMENT_SIZE_OFFSET] =
                i32::try_from(st_input.z)?;
            self.physical_memory[segment_address + SEGMENT_PAGE_TABLE_OFFSET] =
                i32::from(st_input.f);
        }

        for pt_input in page_table_inputs {
            let pt_frame_num = self.physical_memory
                [usize::from(pt_input.s) * SEGMENT_WORD_COUNT + SEGMENT_PAGE_TABLE_OFFSET];

            let page_offset = usize::from(pt_input.p);

            if pt_frame_num < 0 {
                self.disk[usize::try_from(pt_frame_num.abs())?][page_offset] =
                    i32::from(pt_input.f);
            } else {
                self.physical_memory[usize::try_from(pt_frame_num)? * PAGE_SIZE + page_offset] =
                    i32::from(pt_input.f);
            }
        }

        Ok(())
    }

    fn find_free_page(&self) -> VMResult<usize> {
        for (i, page) in self
            .physical_memory
            .chunks(PAGE_SIZE)
            .enumerate()
            .skip(SEGMENT_WORD_COUNT)
        {
            if page.iter().all(|&x| x == 0) {
                return Ok(i);
            }
        }

        Err(VMError::MemoryFull)
    }

    /// Initializes the virtual memory with the given segmentation and page table inputs.
    ///
    /// # Errors
    ///
    /// - `VMError::VirtualAddressOutOfBounds` if the virtual address is out of bounds.
    /// - `VMError::MemoryNotInitialized` if the memory is not initialized.
    /// - `VMError::GeneralError` if an error occurs while converting the virtual address to an
    pub fn translate(&mut self, virtual_address: VirtualAddress) -> VMResult<Address> {
        let segment_address: usize = usize::from(virtual_address.s) * SEGMENT_WORD_COUNT;
        let segment_size = self.physical_memory[segment_address + SEGMENT_SIZE_OFFSET];

        if virtual_address.pw >= u32::try_from(segment_size)? {
            return Err(VMError::VirtualAddressOutOfBounds);
        }

        let page_table_offset = match self
            .physical_memory
            .get(segment_address + SEGMENT_PAGE_TABLE_OFFSET)
        {
            Some(&offset) => match offset {
                offset if offset < 0 => {
                    let disk_offset = usize::try_from(offset.abs())?;
                    let free_page_offset = self.find_free_page()?;

                    self.physical_memory[segment_address + SEGMENT_PAGE_TABLE_OFFSET] =
                        i32::try_from(free_page_offset)?;

                    // Copy Frame From Disk to Memory
                    for (i, &page) in self.disk[disk_offset].iter().enumerate() {
                        self.physical_memory[free_page_offset * PAGE_SIZE + i] = page;
                    }

                    free_page_offset
                }
                0 => return Err(VMError::MemoryNotInitialized),
                offset => usize::try_from(offset)?,
            },
            None => return Err(VMError::VirtualAddressOutOfBounds),
        };

        let page_address = page_table_offset * PAGE_SIZE + usize::from(virtual_address.p);

        let page_offset = match self.physical_memory.get(page_address) {
            Some(&offset) => {
                match offset {
                    offset if offset < 0 => {
                        let disk_offset = usize::try_from(offset.abs())?;
                        let free_page_offset = self.find_free_page()?;

                        self.physical_memory[page_address] = i32::try_from(free_page_offset)?;

                        // Copy Frame From Disk to Memory
                        for (i, &page) in self.disk[disk_offset].iter().enumerate() {
                            self.physical_memory[free_page_offset * PAGE_SIZE + i] = page;
                        }

                        free_page_offset
                    }
                    0 => {
                        return Err(VMError::MemoryNotInitialized);
                    }
                    offset => usize::try_from(offset)?,
                }
            }
            None => return Err(VMError::VirtualAddressOutOfBounds),
        };

        Ok(u32::try_from(
            page_offset * PAGE_SIZE + usize::from(virtual_address.w),
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

        let mut virtual_memory = VirtualMemory::new();
        virtual_memory
            .init(st_inputs, pt_inputs)
            .expect("Failed to init");

        virtual_memory
    }

    #[test]
    fn init() {
        let virtual_memory = before();

        assert_eq!(
            virtual_memory.physical_memory[8 * SEGMENT_WORD_COUNT + SEGMENT_SIZE_OFFSET],
            4000
        );
        assert_eq!(
            virtual_memory.physical_memory[8 * SEGMENT_WORD_COUNT + SEGMENT_PAGE_TABLE_OFFSET],
            3
        );
        assert_eq!(
            virtual_memory.physical_memory[9 * SEGMENT_WORD_COUNT + SEGMENT_SIZE_OFFSET],
            5000
        );
        assert_eq!(
            virtual_memory.physical_memory[9 * SEGMENT_WORD_COUNT + SEGMENT_PAGE_TABLE_OFFSET],
            -7
        );

        assert_eq!(virtual_memory.disk[7][0], 13);
        assert_eq!(virtual_memory.disk[7][1], -25);

        assert_eq!(virtual_memory.physical_memory[3 * PAGE_SIZE], 10);
        assert_eq!(virtual_memory.physical_memory[3 * PAGE_SIZE + 1], -20);
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
        let mut vm = VirtualMemory::new();

        let free_page = vm.find_free_page().expect("Failed to find free page");
        assert_eq!(free_page, 2);

        vm.physical_memory[PAGE_SIZE * 2] = 1;

        let free_page = vm.find_free_page().expect("Failed to find free page");
        assert_eq!(free_page, 3);
    }

    #[test]
    fn find_free_page_full() {
        let mut vm = VirtualMemory::new();

        for i in 0..PAGE_COUNT {
            vm.physical_memory[i * PAGE_SIZE] = 1;
        }

        let free_page = vm.find_free_page();
        assert_eq!(free_page.unwrap_err(), VMError::MemoryFull);
    }
}
