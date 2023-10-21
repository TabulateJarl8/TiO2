use log::{debug, error};

use crate::utils::copy_into_index;

use super::{
    common::FILE_HEADER,
    tokens::{get_inverse_tokens_as_str, Byte},
};

/// Calculate the bytes and carry bit for a given size.
///
/// The maximum filesize is 255*255. Since one byte can only hold 255, we have a size followed by a carry byte.
///
/// # Arguments
///
/// * `size` - The size of the file.
///
/// # Returns
///
/// A result containing an array of two u8 bytes representing the file size.
///
/// # Errors
///
/// Returns an error if the provided size exceeds the absolute limit.
pub fn int_to_bytes(size: usize) -> Result<[u8; 2], anyhow::Error> {
    let mut bytes: [u8; 2] = [0; 2];

    // size byte
    bytes[0] = (size & 0xFF) as u8;
    // carry byte
    bytes[1] = ((size >> 8) & 0xFF) as u8;

    Ok(bytes)
}

/// Create a metadata header and footer for a TI-8XP program.
///
/// This function generates a metadata header and footer (checksum) for a TI-8XP program.
///
/// [This](https://github.com/SmellyModder/TI8xp/blob/main/src/main/java/net/smelly/tieightxp/TIPrgmCompiler.java#L105) GitHub project was a good reference in making this.
///
/// # Arguments
///
/// * `ti_basic_data` - The main body bytes of an 8XP program.
/// * `program_name` - The name of the program.
///
/// # Returns
///
/// A result containing a tuple of two arrays. The first array is of 74 u8 bytes representing the metadata header, and the second array is of 2 u8 bytes representing the footer.
///
/// # Errors
///
/// Returns an error if the header length is incorrect.
pub fn create_metadata(
    ti_basic_data: &Vec<u8>,
    program_name: &str,
) -> Result<([u8; 74], [u8; 2]), anyhow::Error> {
    let mut header: [u8; 74] = [0x0; 74];
    let mut index_pointer: usize = 0;

    // initialize with the 10 bytes that every file has
    index_pointer = copy_into_index(&mut header, &FILE_HEADER, index_pointer);

    //  Appends a comment area of metadata to the header
    //  Follows the form [NULL]40 characters[NULL]character[NULL][NULL][hex code][NEWLINE]
    //  If the comment contains fewer than 40 characters, the unused
    //  characters are filled with null characters.  It appears that
    //  more than 40 characters can be put here, but then the hex codes
    //  at the end change. It doesn't seem to do anything,
    //  but with over 40 characters it doesn't seem to be needed.
    //  So, using the extra characters this section of the header becomes
    //  [NULL]comment string, 42 chars[DC4][NULL][NEWLINE]

    //  I've seen other people use 0xA here, so maybe that would fix it if it breaks
    header[index_pointer] = 0x0;
    index_pointer += 1;

    index_pointer = copy_into_index(
        &mut header,
        b"File compiled by TiO2 from TabulateJarl8\x00\x00",
        index_pointer,
    );

    // data.len() + 19, carry byte
    debug!("Size + 19: {}", ti_basic_data.len() + 19);
    index_pointer = copy_into_index(
        &mut header,
        &int_to_bytes(ti_basic_data.len() + 19)?,
        index_pointer,
    );

    index_pointer = copy_into_index(
        &mut header,
        &[
            // some junk im not sure about
            0xD, 0x0,
        ],
        index_pointer,
    );

    let size = ti_basic_data.len() + 2;
    index_pointer = copy_into_index(&mut header, &int_to_bytes(size)?, index_pointer);

    // 0x05 means editable program, 0x06 means uneditable
    header[index_pointer] = 0x05;
    index_pointer += 1;

    // append truncated name to end of header
    // name is 8 characters long at most, and its followed by 2 NULL characters,
    // so we create an array of length 10 (8 + 2)
    let mut string_bytes: [u8; 10] = [0x0; 10];
    let truncated_program_name = if program_name.len() > 8 {
        program_name.to_ascii_uppercase().chars().take(8).collect()
    } else {
        program_name.to_ascii_uppercase()
    };

    debug!("truncated name: {}", truncated_program_name);

    copy_into_index(&mut string_bytes, truncated_program_name.as_bytes(), 0);
    index_pointer = copy_into_index(&mut header, &string_bytes, index_pointer);

    // Adding the size a second time as it is repeated after the name
    index_pointer = copy_into_index(&mut header, &int_to_bytes(size)?, index_pointer);

    index_pointer = copy_into_index(&mut header, &int_to_bytes(size - 2)?, index_pointer);

    if index_pointer != 74 {
        return Err(anyhow::Error::msg(format!(
            "Fatal error: header length is incorrect: {}",
            index_pointer
        )));
    }

    let checksum = [ti_basic_data, &header[55..]]
        .concat()
        .iter()
        .map(|&x| x as u32)
        .sum::<u32>() as usize;

    let footer = int_to_bytes(checksum)?;

    debug!("Generated header: {:x?}", header);
    debug!("Generated footer: {:x?}", footer);

    Ok((header, footer))
}

/// Compile a Vec of strings into a Vec of bytes, representing a TI-8XP bytecode program.
///
/// This function takes a `Vec` of `&str` containing the source code lines and attempts to convert
/// it into a sequence of bytes that represent a bytecode program. It replaces the `→` character
/// with `->` in the input and uses a provided set of tokens to map substrings to bytes.
///
/// # Arguments
///
/// * `file_contents`: A `Vec` of `&str` containing the source code lines.
///
/// # Returns
///
/// A `Result` containing a `Vec` of `u8` bytes representing the bytecode program if compilation
/// is successful, or an `anyhow::Error` if an error occurs during compilation.
///
/// # Examples
///
/// ```
/// use tio2::translation::compile::compile_to_bytecode;
///
/// todo!();
/// ```
///
pub fn compile_to_bytecode(file_contents: Vec<&str>) -> Result<Vec<u8>, anyhow::Error> {
    let program_string = file_contents.join("\n").replace('→', "->");

    let tokens = get_inverse_tokens_as_str();

    // keep track of when we're in strings for parsing
    let mut in_string = false;

    let longest_program_string = tokens.keys().map(|k| k.len()).max().unwrap();

    let mut program_data: Vec<&Byte> = Default::default();

    let mut current_char = 0;

    while current_char < program_string.len() {
        let mut found = false;
        let mut chars_further = longest_program_string;

        if in_string {
            // match the first character
            // this might break, but i think we have all uppercase/lowercase
            // covered so hopefully it'll be fine
            chars_further = 1;
        }

        // Greedily start with the maximum size string we have and back
        // down until we get to something that we can create a token from.
        while !found && chars_further > 0 {
            let sliced_string: String = program_string
                .chars()
                .take(current_char + chars_further)
                .skip(current_char)
                .collect();

            // entering or exiting a string
            if sliced_string.starts_with('"') {
                in_string = !in_string;
            }

            // end of line, strings close automatically
            if sliced_string.starts_with('\n') {
                in_string = false;
            }

            match tokens.get(sliced_string.as_str()) {
                Some(token) => {
                    found = true;
                    debug!("token: {:?}", sliced_string);
                    program_data.push(token);
                    current_char += chars_further;
                }
                None => chars_further -= 1,
            }

            if chars_further == 0 {
                error!("remainder: {:?}", &program_string[current_char..]);
                return Err(anyhow::Error::msg(
                    "Something went horribly wrong while compiling.",
                ));
            }
        }
    }

    let program_data_bytes = program_data
        .iter()
        .flat_map(|byte| match byte {
            Byte::Single(val) => vec![*val].into_iter(),
            Byte::Double(arr) => arr.to_vec().into_iter(),
        })
        .collect::<Vec<u8>>();

    debug!("Generated data bytes: {:x?}", program_data_bytes);

    Ok(program_data_bytes)
}
