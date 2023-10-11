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
