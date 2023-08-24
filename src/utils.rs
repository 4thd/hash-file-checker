use blake2::Blake2b;
use crc32fast::Hasher;
use md5;
use sha1::{Digest, Sha1};
use sha2::Sha256;
use sha3::Sha3_256;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Result};

pub fn list_files() -> Result<()> {
    let entries = fs::read_dir(".")?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    println!("=> {:?}", file_name_str);
                }
            }
        }
    }
    Ok(())
}

pub fn calculate_md5(path: &str) -> Result<String> {
    let file = File::open(path.trim())?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    let digest = md5::compute(buffer);

    Ok(format!("{:x}", digest))
}

pub fn calculate_sha1(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha1::new();
    let mut buffer = [0; 1024];

    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

pub fn calculate_sha3(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha3_256::new();
    let mut buffer = [0; 1024];

    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

pub fn calculate_sha256(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

pub fn calculate_blake2(path: &str) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut hasher = Blake2b::new();
    let mut buffer = [0; 1024];

    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

pub fn calculate_crc32(file_path: &str) -> Result<u32> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);

    let mut hasher = Hasher::new();
    let mut buffer = [0; 1024];

    loop {
        let read = reader.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }

    Ok(hasher.finalize())
}
