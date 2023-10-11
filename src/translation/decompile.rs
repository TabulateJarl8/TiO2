//! This module defines functionality for working with TI-8XP files, including reading and decompiling them.
//! The primary struct, [`TIFile`], represents the structure of a TI-8XP file.
//! The primary function that should be used in this module is [`decompile`]

use log::{debug, error};

use crate::translation::{
    common::{self, TIFile},
    tokens,
};

/// Checks if the given header is a valid TI 8XP header.
///
/// The TI-8XP header should match the following byte sequence:
/// ```plaintext
/// **TI83F*\x1a\n
/// ```
/// or, in hex:
/// ```plaintext
/// 0x2A 0x2A 0x54 0x49 0x38 0x33 0x46 0x2A 0x1A 0xA
/// ```
///
/// # Arguments
///
/// * `header` - A 74-byte array representing the header of a TI-8XP file.
///
/// # Returns
///
/// Returns true if the header is valid, otherwise false.
pub fn valid_8xp_header(header: [u8; 74]) -> bool {
    header[..10] == common::FILE_HEADER
}

/// Reads binary data and constructs a `TIFile` struct from it.
///
/// # Arguments
///
/// * `data` - A vector of bytes containing binary data from a TI-8XP file.
///
/// # Returns
///
/// Returns a `Result` containing a `TIFile` if successful, or an error if the data is invalid.
pub fn read_binary_data(data: Vec<u8>) -> Result<TIFile, anyhow::Error> {
    if data.len() < 74 {
        debug!("{:?}", data);
        return Err(anyhow::Error::msg(
            "File is only long enough to contain metadata.",
        ));
    }

    let mut header: [u8; 74] = [0; 74];
    header.clone_from_slice(&data[..74]);

    if !valid_8xp_header(header) {
        debug!("{:?}", &header[..10]);
        return Err(anyhow::Error::msg(
            "file is not a valid 8XP file: invalid header",
        ));
    }

    let data: Vec<u8> = data[74..data.len() - 2].to_vec();
    let footer: Vec<u8> = data[data.len() - 2..data.len()].to_vec();

    Ok(TIFile {
        header,
        data,
        footer,
    })
}

/// Decompiles a TI-8XP file into a vector of strings representing the lines of the decompiled content.
///
/// # Arguments
///
/// * `data` - A vector of bytes containing binary data from a TI-8XP file.
///
/// # Returns
///
/// Returns a `Result` containing a vector of strings if successful, or an error if the decompilation fails.
pub fn decompile(data: Vec<u8>) -> Result<Vec<String>, anyhow::Error> {
    let ti_data = read_binary_data(data)?;
    debug!("{:x?}", ti_data);

    let mut plaintext = String::new();
    let single_tokens = &tokens::SINGLE_BYTE_TOKENS;
    let double_tokens = &tokens::DOUBLE_BYTE_TOKENS;

    let mut byte_num = 0;
    while byte_num < ti_data.data.len() {
        let curr_byte = ti_data.data[byte_num];

        // If the current byte exists in the tokens, see if we
        // can find a more specific one (2 bytes) that matches. If not, use
        // the first. We only need to worry about up to 2 bytes.
        if let Some(single_token) = single_tokens.get(&curr_byte) {
            if byte_num + 1 < ti_data.data.len() {
                if let Some(double_token) =
                    double_tokens.get(&[curr_byte, ti_data.data[byte_num + 1]])
                {
                    plaintext.push_str(double_token);
                    byte_num += 2;
                } else {
                    plaintext.push_str(single_token);
                    byte_num += 1;
                }
            } else {
                plaintext.push_str(single_token);
                byte_num += 1;
            }
        } else if byte_num + 1 < ti_data.data.len() {
            // If the current byte is not in the tokens, see if we can add
            // on the next byte to make it work. If so, use that, otherwise
            // spit out an error but do the rest.
            match double_tokens.get(&[curr_byte, ti_data.data[byte_num + 1]]) {
                Some(token) => {
                    plaintext.push_str(token);
                    byte_num += 2;
                }
                None => {
                    error!("Could not decode {:x?}", curr_byte);
                    error!("Next byte: {:x?}", ti_data.data.get(byte_num + 1));
                    byte_num += 1;
                }
            }
        }
    }

    Ok(plaintext.split('\n').map(str::to_string).collect())
}
