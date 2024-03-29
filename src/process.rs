use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
};

use virtual_memory::{
    error::{VMError, VMResult},
    io::{pt_input::PTInput, st_input::STInput},
    vm::{virtual_address::VirtualAddress, virtual_memory::VirtualMemory},
};

fn process_init(file_path: PathBuf) -> VMResult<VirtualMemory> {
    let f = File::open(file_path)?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    reader.read_line(&mut line)?;

    let st_inputs: Vec<STInput> = line
        .split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            STInput::new(chunk[0], chunk[1], chunk[2]).expect("Invalid st_input in init file")
        })
        .collect();

    line.clear();
    reader.read_line(&mut line)?;

    let pt_inputs: Vec<PTInput> = line
        .split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            PTInput::new(chunk[0], chunk[1], chunk[2]).expect("Invalid pt_input in init file")
        })
        .collect();

    let virtual_memory =
        VirtualMemory::new(st_inputs, pt_inputs).expect("Failed to initialize virtual memory");

    Ok(virtual_memory)
}

pub fn process(
    init_file_path: PathBuf,
    input_file_path: PathBuf,
    output_file_path: PathBuf,
) -> VMResult<()> {
    let mut virtual_memory = process_init(init_file_path)?;

    let mut input_file = File::open(input_file_path)?;

    let mut input_data = String::new();
    input_file.read_to_string(&mut input_data)?;

    let output_data: String = input_data
        .split_whitespace()
        .map(|address| {
            let virtual_address = VirtualAddress::new(address.parse().expect("Invalid Input Data"))
                .expect("Invalid Data");

            match virtual_memory.translate(virtual_address) {
                Ok(physical_address) => physical_address.to_string(),
                Err(VMError::VirtualAddressOutOfBounds | VMError::MemoryNotInitialized) => {
                    (-1).to_string()
                }
                Err(error) => panic!("{:?}", error),
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    let mut output_file = File::create(output_file_path)?;
    writeln!(output_file, "{}", output_data)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn basic() {
        process(
            "test-data/init.txt".into(),
            "test-data/input.txt".into(),
            "test-data/output.tmp".into(),
        )
        .expect("Failed to process basic example");

        let expected_output =
            read_to_string("test-data/output.txt").expect("Failed to read expected output");
        let output = read_to_string("test-data/output.tmp").expect("Failed to read output");

        assert_eq!(expected_output, output);
    }

    #[test]
    fn error() {
        process(
            "test-data/error_init.txt".into(),
            "test-data/error_input.txt".into(),
            "test-data/error_output.tmp".into(),
        )
        .expect("Failed to process error example");

        let expected_output =
            read_to_string("test-data/error_output.txt").expect("Failed to read expected output");
        let output = read_to_string("test-data/error_output.tmp").expect("Failed to read output");

        assert_eq!(expected_output, output);
    }
}
