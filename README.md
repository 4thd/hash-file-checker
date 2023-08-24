# HASH FILE CHECKER

*A simple Rust program to verify file hash integrity.*

## Description

This program allows a user to:

1. View all files in the current directory.
2. Input the name of a file and its expected hash.
3. Calculate hashes for the chosen file using various algorithms:
   - MD5
   - SHA-1
   - SHA-3
   - SHA-256
   - Blake2
   - CRC32
4. Compare the calculated hashes to the user's expected hash.
5. Highlight matches, and verify the file if any match. Otherwise, display a warning.

## Features

- **Current Path Detection:** Display the current working directory path.
- **File Listing:** Lists all files in the current directory.
- **User Input:** Takes in a file name and its expected hash.
- **Hash Verification:** Verifies against several algorithms:
  - MD5
  - SHA-1
  - SHA-3
  - SHA-256
  - Blake2
  - CRC32

## Dependencies

- A `utils` module is necessary, providing utility functions like file listing and hash calculation.

## Usage

1. Compile and run the program.
2. It displays all files in the current directory.
3. Enter the file name to check.
4. Input the expected hash for the file.
5. Program displays calculated hashes for various algorithms.
6. Verification message is shown if there's a match; otherwise, a warning appears.

## Notes

- Uses ANSI escape codes for colored terminal output.
- Ensure the `utils` module is in the same project.

## Installation & Running with Cargo

### Pre-requisites:

- Rust and Cargo. If not installed, get them from [Rust's official website](https://rust-lang.org/).

### Instructions:

1. **Compile**: Navigate to the project directory and compile with:

```sh
cargo build --release
```

2. **Run**: Execute the program using:

```sh
cargo run
```