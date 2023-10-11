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

/// The header for TI-8XP files
pub const FILE_HEADER: [u8; 10] = [0x2A, 0x2A, 0x54, 0x49, 0x38, 0x33, 0x46, 0x2A, 0x1A, 0xA];
