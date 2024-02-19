use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Read, Write},
    path::PathBuf,
};

use virtual_memory::{
    error::{VMError, VMResult},
    io::{pt_input::PTInput, st_input::STInput},
    vm::{virtual_address::VirtualAddress, virtual_memory::VirtualMemory},
};

fn process_init(file_path: PathBuf) -> VMResult<VirtualMemory> {
    let f = File::open(file_path).map_err(|_| VMError::IOError)?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();
    reader.read_line(&mut line).map_err(|_| VMError::IOError)?;

    // Chunks of 3 &str elements
    let st_inputs: Vec<STInput> = line
        .split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            STInput::new(chunk[0], chunk[1], chunk[2]).expect("Invalid st_input in init file")
        })
        .collect();

    line.clear();
    reader.read_line(&mut line).map_err(|_| VMError::IOError)?;

    let pt_inputs: Vec<PTInput> = line
        .split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            PTInput::new(chunk[0], chunk[1], chunk[2]).expect("Invalid pt_input in init file")
        })
        .collect();

    let mut virtual_memory = VirtualMemory::new();
    virtual_memory
        .init(st_inputs, pt_inputs)
        .expect("Failed to initialize virtual memory");

    Ok(virtual_memory)
}

pub fn process(
    init_file_path: PathBuf,
    input_file_path: PathBuf,
    output_file_path: PathBuf,
) -> VMResult<()> {
    let mut virtual_memory = process_init(init_file_path)?;

    let output_file = File::create(output_file_path).map_err(|_| VMError::IOError)?;
    let mut writer = BufWriter::new(output_file);

    let mut input_file = File::open(input_file_path).map_err(|_| VMError::IOError)?;

    let mut input_data = String::new();
    input_file
        .read_to_string(&mut input_data)
        .map_err(|_| VMError::IOError)?;

    let output_data: String = input_data
        .trim()
        .split_whitespace()
        .map(|address| {
            let virtual_address =
                VirtualAddress::new(address.parse().expect("Invalid Data")).expect("Invalid Data");
            virtual_memory
                .translate(virtual_address)
                .expect("Invalid Data")
        })
        .map(|address| address.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    writeln!(writer, "{}", output_data).map_err(|_| VMError::IOError)?;

    Ok(())
}
