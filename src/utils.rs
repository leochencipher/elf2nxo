use crypto_hash::{Algorithm, Hasher};
use elf;
use lz4_sys;
use std;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::process;

pub fn add_padding(vec: &mut Vec<u8>, padding: usize) -> () {
    let real_size = vec.len();
    vec.resize(((real_size as usize) + padding) & !padding, 0);
}

pub fn get_section_data(
    file: &mut File,
    header: &elf::types::ProgramHeader,
) -> std::io::Result<Vec<u8>> {
    let mut data = vec![0; header.filesz as usize];
    file.seek(SeekFrom::Start(header.offset))?;
    file.read(&mut data)?;
    Ok(data)
}

// TODO: make compression level configurable
pub fn compress(uncompressed_data: &mut Vec<u8>) -> Vec<u8> {
    let uncompressed_data_size = uncompressed_data.len() as i32;
    let max_compression_size = unsafe { lz4_sys::LZ4_compressBound(uncompressed_data_size) };

    // Create res vector and make sure the max memory needed is availaible
    let mut res: Vec<u8> = Vec::new();
    res.resize(max_compression_size as usize, 0);

    let res_code = unsafe {
        lz4_sys::LZ4_compress_default(
            uncompressed_data.as_mut_ptr(),
            res.as_mut_ptr(),
            uncompressed_data_size,
            max_compression_size,
        )
    };

    if res_code <= 0 {
        println!("Error: LZ4 compression function returned {}", res_code);
        process::exit(1)
    } else {
        res.resize(res_code as usize, 0);
        res
    }
}

pub fn calculate_sha256(data: &Vec<u8>) -> std::io::Result<Vec<u8>> {
    let mut hasher = Hasher::new(Algorithm::SHA256);
    hasher.write(data)?;
    Ok(hasher.finish())
}
