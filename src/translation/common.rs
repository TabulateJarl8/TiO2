//! The `common` module provides a helper struct, [`TIFile`], which is used in both decompilation and compilation
//! for managing TI-84 Plus calculator files in the 8XP format. It includes
//! methods for writing data to a file in the appropriate format and extracting
//! the program name from the file header.

use std::{fs::File, io::Write, string::FromUtf8Error};

/// A helper struct for managing TI-84 Plus calculator files (8XP format).
///
/// [`TIFile`] represents a file in the 8XP format used by TI-84 Plus calculators. It provides
/// methods for writing data to a file and extracting the program name from the file header.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct TIFile {
    /// The header that provides information on the compiled file
    pub header: [u8; 74],
    /// The core file data
    pub data: Vec<u8>,
    /// The 2-byte footer (file checksum)
    pub footer: Vec<u8>,
}

impl TIFile {
    /// Write the content of the [`TIFile`] to a file with the appropriate extension.
    ///
    /// This method creates a file with the program's name in the 8XP format used by TI-84 Plus
    /// calculators. It writes the header, data, and footer to the file.
    ///
    /// # Returns
    ///
    /// A [`Result`] indicating success if the data was successfully written to the file,
    /// or an [`anyhow::Error`] if an error occurred.
    pub fn write_to_file(&self) -> Result<(), anyhow::Error> {
        let program_name = self.extract_program_name()?;
        let mut f = File::create(program_name + ".8XP")?;
        f.write_all(&self.header)?;
        f.write_all(&self.data)?;
        f.write_all(&self.footer)?;

        Ok(())
    }

    /// Extract the program name from the file header.
    ///
    /// This method extracts the program name from the header of the [`TIFile`] and returns it as a
    /// [`String`]. It trims any NULL bytes from the result.
    ///
    /// # Returns
    ///
    /// A [`Result`] containing the program name as a [`String`] if successful, or a
    /// [`FromUtf8Error`] if the extraction fails.
    pub fn extract_program_name(&self) -> Result<String, FromUtf8Error> {
        let result = String::from_utf8(self.header[60..68].to_vec())?;

        // String NULL bytes
        Ok(result.trim_matches(char::from(0)).to_string())
    }
}

/// The 10-byte header for TI-8XP files
pub const FILE_HEADER: [u8; 10] = [0x2A, 0x2A, 0x54, 0x49, 0x38, 0x33, 0x46, 0x2A, 0x1A, 0xA];
