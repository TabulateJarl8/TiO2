use crate::errors;

#[derive(Debug, Clone, Copy)]
pub struct Lbl {
    pub name: [u8; 2],
    pub size: u8,
    pub memory_position: usize,
}

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub bytes: Vec<u8>,
    pub labels: Vec<Lbl>,
    pub bytes_pointer: usize,
}

impl Interpreter {
    pub fn new(bytes_list: Vec<u8>) -> Self {
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
        lbl_map.push(Lbl {
            name: label_name,
            // calculates the label name size. if it's only one byte, this will be calculated to 1, otherwise, 2
            size: 2 - (label_name[1] == 0) as u8,
            memory_position: address,
        });
    }

    println!("{:?}", lbl_map);

    Ok(lbl_map)
}
