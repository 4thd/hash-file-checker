use std::io;
mod utils;
use std::env;

fn main() {
    let mut is_verified = false;

    println!("** HASH CHECKER **\n");

    match env::current_dir() {
        Ok(path) => println!("Current path: {}", path.display()),
        Err(e) => eprint!("Error fecthing current path: {}", e),
    }

    println!("Files in current path:");
    let _ = utils::list_files();

    println!("\nSelect a file: ");
    let mut file = String::new();
    io::stdin()
        .read_line(&mut file)
        .expect("Error reading file");
    let file = file.trim();

    println!("Expected hash: ");
    let mut expected_hash = String::new();
    io::stdin()
        .read_line(&mut expected_hash)
        .expect("Error fetching expected hash");
    let expected_hash = expected_hash.to_string();

    println!("RESULT:");
    // md5 check
    match utils::calculate_md5(&file) {
        Ok(hash) if hash.to_string().trim() == expected_hash.trim() => {
            println!("{}MD5 => {}{}", "\x1b[32m", hash, "\x1b[0m");
            is_verified = true;
        }
        Ok(hash) => println!("MD5 => {}", hash),
        Err(e) => println!("{}", e),
    }
    // sha-1 check
    match utils::calculate_sha1(&file) {
        Ok(hash) if hash.to_string().trim() == expected_hash.trim() => {
            println!("{}SHA-1 => {}{}", "\x1b[32m", hash, "\x1b[0m");
            is_verified = true;
        }
        Ok(hash) => println!("SHA-1 => {}", hash),
        Err(e) => println!("{}", e),
    }

    // sha-3 check
    match utils::calculate_sha3(&file) {
        Ok(hash) if hash.to_string().trim() == expected_hash.trim() => {
            println!("{}SHA-3 => {}{}", "\x1b[32m", hash, "\x1b[0m");
            is_verified = true;
        }
        Ok(hash) => println!("SHA-3 => {}", hash),
        Err(e) => println!("{}", e),
    }
    // sha-256 check
    match utils::calculate_sha256(&file) {
        Ok(hash) if hash.to_string().trim() == expected_hash.trim() => {
            println!("{}SHA-256 => {}{}", "\x1b[32m", hash, "\x1b[0m");
            is_verified = true;
        }
        Ok(hash) => println!("SHA-256 => {}", hash),
        Err(e) => println!("{}", e),
    }

    // blake2 check
    match utils::calculate_blake2(&file) {
        Ok(hash) if hash.to_string().trim() == expected_hash.trim() => {
            println!("{}Blake2 => {}{}", "\x1b[32m", hash, "\x1b[0m");
            is_verified = true;
        }
        Ok(hash) => println!("Blake2 => {}", hash),
        Err(e) => println!("{}", e),
    }

    match utils::calculate_crc32(&file) {
        Ok(hash) if hash.to_string().trim() == expected_hash.trim() => {
            println!("{}CRC32 => {}{}", "\x1b[32m", hash, "\x1b[0m");
            is_verified = true;
        }
        Ok(hash) => println!("CRC32 => {}\n", hash),
        Err(e) => println!("{}", e),
    }

    if !is_verified {
        println!("\x1b[31m[WARNING]\x1b[0m : File hash is not verified!!!");
    } else {
        println!("\x1b[32m[VERIFIED]\x1b[0m : File hash is verified!!!");
    }
}
