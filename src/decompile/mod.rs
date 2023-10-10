pub mod tokens;

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

fn read_binary_data(data: Vec<u8>) -> Result<TIFile, anyhow::Error> {
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
