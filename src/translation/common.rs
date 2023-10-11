use std::{string::FromUtf8Error, fs::File, io::Write};

/// Represents the structure of a TI-8XP file
#[derive(Debug, Clone)]
pub struct TIFile {
    /// The header that provides information on the compiled file
    pub header: [u8; 74],
    /// The core file data
    pub data: Vec<u8>,
    /// The 3-byte footer (data seems to be useless)
    pub footer: Vec<u8>,
}

impl TIFile {
    pub fn write_to_file(&self) -> Result<(), anyhow::Error> {
        let program_name = self.extract_program_name()?;
        let mut f = File::create(program_name + ".8XP")?;
        f.write_all(&self.header)?;
        f.write_all(&self.data)?;
        f.write_all(&self.footer)?;

        Ok(())
    }

    pub fn extract_program_name(&self) -> Result<String, FromUtf8Error> {
        let result = String::from_utf8(self.header[60..68].to_vec())?;

        // String NULL bytes
        Ok(result.trim_matches(char::from(0)).to_string())
    }
}

/// The header for TI-8XP files
pub const FILE_HEADER: [u8; 10] = [0x2A, 0x2A, 0x54, 0x49, 0x38, 0x33, 0x46, 0x2A, 0x1A, 0xA];
