//! This file contains structs and functions used for handling the Lbl token.

use crate::{errors, utils};

/// Represents a label in the TI-BASIC bytecode format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Lbl {
    /// The NULL byte padded hex name of the label
    pub name: [u8; 2],
    /// The memory address of the instruction after the `\n` after the Lbl name
    ///
    /// WARNING: this can be out of bounds of the array length, so we need to check when
    /// accessing it
    pub skip_to_memory_position: usize,
}

/// Searches for labels within a list of bytes and returns them as a result.
///
/// Mainly used internally, but can be used in other scenarios if needed.
///
/// # Arguments
///
/// * `bytes_list` - A reference to a vector of bytes representing the custom bytecode.
///
/// # Returns
///
/// A `Result` containing a vector of `Lbl` if labels are found, or an error if any issues occur.
///
/// # Example
///
/// ```
/// use tio2::interpreter::{Interpreter, Lbl};
///
/// // Basic infinite loop program
/// let bytecode = vec![0xd6, 0x41, 0x3f, 0xde, 0x2a, 0x41, 0x2a, 0x3f, 0xd7, 0x41];
/// let interpreter = Interpreter::new(&bytecode).unwrap();
///
/// assert_eq!(interpreter.labels, vec![Lbl { name: [65, 0], skip_to_memory_position: 3 }]);
/// ```
pub fn find_labels(bytes_list: &Vec<u8>) -> Result<Vec<Lbl>, anyhow::Error> {
    let lbl_addresses: Vec<usize> = bytes_list
        .iter()
        .enumerate()
        // 0xD6 is the hex value for "Lbl"
        .filter(|(_, &r)| r == 0xD6)
        .map(|(index, _)| index)
        .collect();

    // TODO: TI-BASIC gives duplicate labels to the first occurance
    let mut lbl_map: Vec<Lbl> = Vec::new();

    for address in lbl_addresses {
        let label_name = get_label_name(bytes_list, address)?;

        // calculates the label name size. if it's only one byte, this will be calculated to 1, otherwise, 2
        let size = 2 - (label_name[1] == 0) as usize;
        // address is the address position of the Lbl text
        // we then skip forward the length of the Lbl name + 2 to get to the next token
        // adding 1 will get us to the '\n' on the same line as the Lbl, and adding 2 will get us
        // to the first token of the next line
        // WARNING: this can be out of bounds of the array length, so we need to check when
        // accessing it
        let next_instruction_address = address + size + 2;

        lbl_map.push(Lbl {
            name: label_name,
            skip_to_memory_position: next_instruction_address,
        });
    }

    println!("{:?}", lbl_map);

    Ok(lbl_map)
}

/// Retrieves the label name from TI-BASIC bytecode at the specified memory address.
///
/// The label name is expected to follow a `Lbl` or `Goto` token in the bytecode.
///
/// # Arguments
///
/// * `bytes_list` - A reference to a vector of bytes representing the TI-BASIC bytecode. This should be the data section, not the header or footer.
/// * `lbl_memory_address` - The memory address where the `Lbl` or `Goto` token is expected to be found. This is not where the label name starts.
///
/// # Returns``
///
/// A `Result` containing a fixed-size array of two bytes representing the label name, or an error
/// if the label name cannot be extracted. If the label name is one byte long, this list will be
/// NULL padded.
/// 
///
/// # Errors
///
/// This function can return an error if it encounters unexpected bytes while parsing the label name.
///
/// # Example
///
/// ```
/// use tio2::interpreter::label::get_label_name;
///
/// // Simple test program that defined Lbl A and then displays "A"
/// let bytecode = vec![0xd6, 0x41, 0x3f, 0xde, 0x2a, 0x41, 0x2a];
///
/// // Extract the label name at memory address 0 (the `Lbl` token).
/// let label_name = get_label_name(&bytecode, 0).unwrap();
///
/// assert_eq!(label_name, [0x41, 0x0]);
/// ```
///
/// # Note
///
/// This function assumes that label names in the bytecode consist of two bytes, where the second
/// byte can be 0x00 to indicate the end of the label name.
pub fn get_label_name(
    bytes_list: &Vec<u8>,
    lbl_memory_address: usize,
) -> Result<[u8; 2], anyhow::Error> {
    // test that lbl_memory_address is in range
    let token_at_address = match bytes_list.get(lbl_memory_address) {
        Some(&v) => v,
        None => {
            return Err(anyhow::Error::msg(format!(
                "Index {} out of range. Bytes list is {} bytes long",
                lbl_memory_address,
                bytes_list.len()
            )))
        }
    };
    // test that the given memory address is for a Lbl or Goto token
    if ![0xD6, 0xD7].contains(&token_at_address) {
        return Err(anyhow::Error::msg(format!(
            "Token `0x{:x?}` is not Lbl (0xD6) or Goto (0xD7)",
            token_at_address
        )));
    }

    // how many bytes to skip when reading label. minimum of one. doesn't include the Lbl byte itself
    let token1 = match bytes_list.get(lbl_memory_address + 1) {
        Some(&token) => {
            if utils::ALPHANUMERIC_RANGE.contains(&token) {
                token
            } else {
                return Err(anyhow::Error::msg(format!(
                    "Unexpected byte following Lbl/Goto: {:x?}",
                    token
                )));
            }
        }
        None => return Err(errors::UnexpectedEOFError::new("Lbl/Goto").into()),
    };

    let token2 = match bytes_list.get(lbl_memory_address + 2) {
        Some(&token) => {
            if utils::ALPHANUMERIC_RANGE.contains(&token) {
                token
            } else if token == 0x3F {
                // newline signifies end of label
                0
            } else {
                // label has invalid character
                return Err(anyhow::Error::msg(format!(
                    "Unexpected byte following Lbl/Goto: {:x?}",
                    token
                )));
            }
        }
        None => {
            // end of file
            0
        }
    };

    Ok([token1, token2])
}
