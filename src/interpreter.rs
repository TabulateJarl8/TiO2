use crate::errors;

/// Represents a label in the TI-BASIC bytecode format.
#[derive(Debug, Clone, Copy)]
pub struct Lbl {
    /// The NULL byte padded hex name of the label
    pub name: [u8; 2],
    /// The memory address of the instruction after the '\n' after the Lbl name
    /// WARNING: this can be out of bounds of the array length, so we need to check when
    /// accessing it
    pub skip_to_memory_position: usize,
}

/// The TI-BASIC bytecode interpreter. Hold information such the instruction stack, Lbl positions, etc.
#[derive(Debug, Clone)]
pub struct Interpreter {
    /// The list of bytes for a given TI-BASIC program
    pub bytes: Vec<u8>,
    /// A Vec of Lbl objects. Contains information on jumping to memory positions
    pub labels: Vec<Lbl>,
    /// The pointer to the current address in the bytes memory
    pub bytes_pointer: usize,
}

impl Interpreter {
    /// Creates a new interpreter with the given byte list.
    ///
    /// It will also locate and store the labels found in the bytecode.
    ///
    /// # Arguments
    ///
    /// * `bytes_list` - A vector of bytes representing the TI-BASIC bytecode.
    ///
    /// # Returns
    ///
    /// A new `Interpreter` instance.
    ///
    /// # Errors
    /// Panics if there is an error in parsing the Lbl instructions.
    pub fn new(bytes_list: Vec<u8>) -> Self {
        // TODO: dont panic
        let labels = find_labels(&bytes_list).unwrap();
        Self {
            bytes: bytes_list,
            labels,
            bytes_pointer: 0,
        }
    }

    pub fn interpret_bytes(&mut self) {}

    pub fn interpret_next_byte(&mut self) {}
}

/// Searches for labels within a list of bytes and returns them as a result.
///
/// # Arguments
///
/// * `bytes_list` - A reference to a vector of bytes representing the custom bytecode.
///
/// # Returns
///
/// A `Result` containing a vector of `Lbl` if labels are found, or an error if any issues occur.
fn find_labels(bytes_list: &Vec<u8>) -> Result<Vec<Lbl>, anyhow::Error> {
    let lbl_addresses: Vec<usize> = bytes_list
        .iter()
        .enumerate()
        // 0xD6 is the hex value for "Lbl"
        .filter(|(_, &r)| r == 0xD6)
        .map(|(index, _)| index)
        .collect();

    // TODO: TI-BASIC gives duplicate labels to the first occurance
    let mut lbl_map: Vec<Lbl> = Vec::new();
    let alphanumeric_range = (0x41..=0x5B).chain(0x30..=0x39).collect::<Vec<u8>>();

    for address in lbl_addresses {
        // how many bytes to skip when reading label. minimum of one. doesn't include the Lbl byte itself
        let token1 = match bytes_list.get(address + 1) {
            Some(&token) => {
                if alphanumeric_range.contains(&token) {
                    token
                } else {
                    return Err(anyhow::Error::msg(format!(
                        "Unexpected byte following Lbl: {:x?}",
                        token
                    )));
                }
            }
            None => return Err(errors::UnexpectedEOFError::new("Lbl").into()),
        };

        let token2 = match bytes_list.get(address + 2) {
            Some(&token) => {
                if alphanumeric_range.contains(&token) {
                    token
                } else if token == 0x3F {
                    // newline signifies end of label
                    0
                } else {
                    // label has invalid character
                    return Err(anyhow::Error::msg(format!(
                        "Unexpected byte following Lbl: {:x?}",
                        token
                    )));
                }
            }
            None => {
                // end of file
                0
            }
        };

        let label_name = [token1, token2];

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
