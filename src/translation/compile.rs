use log::debug;

use crate::utils::copy_into_index;

use super::common::FILE_HEADER;

/// Calculate the size bytes for a given filesize.
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
pub fn create_size_for_header(size: usize) -> Result<[u8; 2], anyhow::Error> {
    const ABSOLUTE_LIMIT: usize = 255 * 255;
    if size > ABSOLUTE_LIMIT {
        return Err(anyhow::Error::msg(format!(
            "File is beyond the allowed size for compiled TI-Basic files: yours: {}, limit: {}",
            size, ABSOLUTE_LIMIT
        )));
    }
    let mut header_size: [u8; 2] = [0; 2];

    // size byte
    header_size[0] = (size - (255 * (size / 255))) as u8;
    // carry byte
    header_size[1] = (size / 255) as u8;

    Ok(header_size)
}

/// Create a metadata header for a TI-8XP program.
///
/// This function generates a metadata header for a TI-8XP program.
///
/// # Arguments
///
/// * `ti_basic_data` - The main body bytes of an 8XP program.
/// * `program_name` - The name of the program.
///
/// # Returns
///
/// A result containing an array of 74 u8 bytes representing the metadata header.
///
/// # Errors
///
/// Returns an error if the header length is incorrect.
pub fn create_header(
    ti_basic_data: &Vec<u8>,
    program_name: &str,
) -> Result<[u8; 74], anyhow::Error> {
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

    //  The comment appears to just be plain ASCII text, so not using
    //  binary for it here.
    header[index_pointer] = 0x0;
    index_pointer += 1;

    index_pointer = copy_into_index(
        &mut header,
        "File compiled by TiO2 from TabulateJarl8".as_bytes(),
        index_pointer,
    );
    index_pointer = copy_into_index(
        &mut header,
        &[
            0x0, 0x0,  // two null bytes
            0x4E, // any character?
            0x0,  // some character
            0xA, 0x0,
        ],
        index_pointer,
    );

    let size = ti_basic_data.len() + 2;
    index_pointer = copy_into_index(&mut header, &create_size_for_header(size)?, index_pointer);

    // denotes start of program name
    header[index_pointer] = 0x05;
    index_pointer += 1;

    // append truncated name to end of header
    // name is 8 characters long at most, and its followed by 2 NULL characters,
    // so we create an array of length 10 (8 + 2)
    let mut string_bytes: [u8; 10] = [0x0; 10];
    let truncated_program_name = if program_name.len() > 8 {
        program_name.chars().take(8).collect()
    } else {
        String::from(program_name)
    };

    debug!("truncated name: {}", truncated_program_name);

    copy_into_index(&mut string_bytes, truncated_program_name.as_bytes(), 0);
    index_pointer = copy_into_index(&mut header, &string_bytes, index_pointer);

    // Adding the size a second time as it is repeated after the name
    index_pointer = copy_into_index(&mut header, &create_size_for_header(size)?, index_pointer);

    index_pointer = copy_into_index(
        &mut header,
        &create_size_for_header(size - 2)?,
        index_pointer,
    );

    if index_pointer != 74 {
        return Err(anyhow::Error::msg(format!(
            "Fatal error: header length is incorrect: {}",
            index_pointer
        )));
    }

    Ok(header)
}
