pub mod tokens;

use std::{
    fs::File,
    io::{BufReader, Read},
};

use log::{debug, error};

#[derive(Debug, Clone)]
pub struct TIFile {
    pub header: [u8; 74],
    pub data: Vec<u8>,
    pub footer: Vec<u8>,
}

fn valid_8xp_header(header: [u8; 74]) -> bool {
    // the array below defined the valid 8xp header mimetype. Equivalent to:
    // **TI83F*\x1a\n
    header[..10] == [42, 42, 84, 73, 56, 51, 70, 42, 26, 10]
}

pub fn read_binary_file(filename: &str) -> Result<TIFile, anyhow::Error> {
    let f = File::open(filename)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    if buffer.len() < 74 {
        debug!("{:?}", buffer);
        return Err(anyhow::Error::msg(
            "File is only long enough to contain metadata.",
        ));
    }

    let mut header: [u8; 74] = [0; 74];
    header.clone_from_slice(&buffer[..74]);

    if !valid_8xp_header(header) {
        debug!("{:?}", &header[..10]);
        return Err(anyhow::Error::msg(
            "file is not a valid 8XP file: invalid header",
        ));
    }

    let data: Vec<u8> = buffer[74..buffer.len() - 2].to_vec();
    let footer: Vec<u8> = buffer[buffer.len() - 2..buffer.len()].to_vec();

    Ok(TIFile {
        header,
        data,
        footer,
    })
}

pub fn decompile(ti_data: TIFile) -> Vec<String> {
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
        if single_tokens.contains_key(&curr_byte) {
            if byte_num + 1 < ti_data.data.len()
                && double_tokens.contains_key(&[curr_byte, ti_data.data[byte_num + 1]])
            {
                plaintext.push_str(double_tokens[&[curr_byte, ti_data.data[byte_num + 1]]]);
                byte_num += 2;
            } else {
                plaintext.push_str(single_tokens[&curr_byte]);
                byte_num += 1;
            }

            continue;
        } else {
            // If the current byte is not in the tokens, see if we can add
            // on the next byte to make it work. If so, use that, otherwise
            // spit out an error but do the rest.

            if byte_num + 1 < ti_data.data.len() {
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
    }

    plaintext.split("\n").map(str::to_string).collect()
}
