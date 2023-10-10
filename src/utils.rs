use std::{
    fs::File,
    io::{BufReader, Read},
};

/// Read a file into a `Vec<u8>`.
///
/// # Errors
///
/// This function will return an error if the file fails to open (`io::Result<File>`) or if the BufReader encounters and error while reading into the vector buffer (`io::Result<usize>`) 
pub fn read_file_bytes(filename: &str) -> Result<Vec<u8>, anyhow::Error> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

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
