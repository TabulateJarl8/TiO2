//! This module contains various utility functions.

use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
};

/// Alphanumeric tokens in TI-BASIC. Includes A-Z, 0-9, and theta.
pub const ALPHANUMERIC_RANGE: [u8; 37] = [
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46,
    0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56,
    0x57, 0x58, 0x59, 0x5a, 0x5b,
];

/// Some tokens are two bytes long. The first byte of these tokens will be one of the bytes in this array.
pub const DOUBLE_BYTE_TOKEN_IDENT: [u8; 11] = [
    0x5C, 0x5D, 0x5E, 0x60, 0x61, 0x62, 0x63, 0xAA, 0xBB, 0xEF, 0x7E,
];

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

/// Reads the contents of a file line by line and returns them as a vector of strings.
///
/// Each line is stored as a separate string in the resulting vector.
///
/// # Arguments
///
/// * `filename`: An object that implements the `AsRef<Path>` trait, representing the path to the file
///
/// # Returns
///
/// A `Result` containing a `Vec<String>` if the file is successfully read, or an `anyhow::Error` if an error occurs during file I/O.
///
/// # Errors
///
/// This function may return an `anyhow::Error` in the following situations:
///
/// - If the file specified by `filename` does not exist or cannot be opened.
/// - If there are issues reading the file content, such as permission or encoding errors.
///
/// # Note
///
/// The function expects the file's content to be valid UTF-8. If the file contains non-UTF-8 data, you may need to handle decoding errors or use a different approach to read the file.
pub fn read_file_lines(filename: impl AsRef<Path>) -> Result<Vec<String>, anyhow::Error> {
    let f = File::open(filename)?;
    let reader = BufReader::new(f);
    Ok(reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect())
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

/// Copies the contents of the source byte slice into the destination byte slice,
/// starting at the specified index in the destination slice.
///
/// # Arguments
///
/// * `dest` - A mutable reference to the destination byte slice where the data will be copied.
/// * `src` - A reference to the source byte slice containing the data to be copied.
/// * `start_index` - The index in the destination slice where the copying will begin.
///
/// # Returns
///
/// The updated index after copying all the elements from the source slice to the destination slice.
///
/// # Examples
///
/// ```
/// use tio2::utils::copy_into_index;
///
/// let mut dest = [0u8; 5];
/// let src = [1u8, 2u8, 3u8];
/// let start_index = 1;
/// let new_index = copy_into_index(&mut dest, &src, start_index);
/// assert_eq!(dest, [0u8, 1u8, 2u8, 3u8, 0u8]);
/// assert_eq!(new_index, start_index + src.len());
/// ```
///
/// # Note
///
/// This function copies the elements from the source slice to the destination slice
/// sequentially, and it does not perform bounds checking. Ensure that the destination slice
/// has enough capacity to accommodate the copied elements.
pub fn copy_into_index(dest: &mut [u8], src: &[u8], mut start_index: usize) -> usize {
    for item in src {
        dest[start_index] = *item;
        start_index += 1;
    }

    start_index
}
