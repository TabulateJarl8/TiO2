//! This module contains various utility functions.

use std::{
    fs::File,
    io::{BufReader, Read},
};

/// Reads the contents of a file and returns them as a vector of bytes.
///
/// # Arguments
///
/// * `filename` - A reference to a string representing the path to the file to be read.
///
/// # Returns
///
/// Returns a `Result` containing a vector of bytes if the file is successfully read, or an error if the file cannot be read.
pub fn read_file_bytes(filename: &str) -> Result<Vec<u8>, anyhow::Error> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Checks if the provided binary data is valid UTF-8 encoded text.
///
/// This function checks if the input bytes are valid UTF-8 encoded text by attempting
/// to decode the bytes as a UTF-8 string. It further validates that the UTF-16 encoding
/// of the string does not contain surrogate code points `(0xD800..=0xDFFF)`, which are
/// invalid in UTF-8 encoding.
///
/// # Arguments
///
/// * `data` - A vector of bytes that may represent UTF-8 encoded text.
///
/// # Returns
///
/// Returns `true` if the data is valid UTF-8 text, otherwise `false`.
pub fn is_utf8(data: Vec<u8>) -> bool {
    match std::str::from_utf8(&data) {
        Ok(v) => {
            for byte in v.encode_utf16() {
                if (0xD800..=0xDFFF).contains(&byte) {
                    return false;
                }
            }
            true
        }
        Err(_) => false,
    }
}

pub fn copy_into_index(dest: &mut [u8], src: &[u8], mut start_index: usize) -> usize {
    for item in src {
        dest[start_index] = *item;
        start_index += 1;
    }

    start_index
}
