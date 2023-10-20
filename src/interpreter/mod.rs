use crate::translation::common::TIFile;

use self::label::{find_labels, Lbl};

pub mod label;

/// The TI-BASIC bytecode interpreter. Hold information such the instruction stack, Lbl positions, etc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interpreter {
    /// The list of bytes for a given TI-BASIC program
    pub bytes: Vec<u8>,
    /// A Vec of Lbl objects. Contains information on jumping to memory positions
    pub labels: Vec<Lbl>,
    /// The pointer to the current address in the bytes memory
    pub bytes_pointer: usize,
}

impl Interpreter {
    /// Creates a new `Interpreter` instance for interpreting TI-BASIC bytecode.
    ///
    /// The `Interpreter` is responsible for managing the execution of TI-BASIC bytecode programs,
    /// including handling label positions and byte instructions.
    ///
    /// # Arguments
    ///
    /// * `ti_program` - A TIFile object that will be read. Technically, it does not need to have
    /// a valid header or footer, just a valid data bytes section.
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `Interpreter` if successful, or an error if any issues occur.
    ///
    /// # Example
    ///
    /// ```
    /// use tio2::{translation::common::TIFile, interpreter::Interpreter};
    ///
    /// // Basic hello world program
    /// // The header and footer do not matter when interpreting
    /// let ti_program = TIFile {
    ///     header: [0; 74],
    ///     data: vec![0xde, 0x2a, 0x48, 0x45, 0x4c, 0x4c, 0x4f, 0x2a],
    ///     footer: vec![],
    /// };
    /// let interpreter_result = Interpreter::new(&ti_program);
    ///
    /// match interpreter_result {
    ///     Ok(interpreter) => {
    ///         // Successfully created an interpreter.
    ///         // You can now use it to interpret TI-BASIC bytecode.
    ///         assert_eq!(interpreter.bytes, ti_program.data);
    ///     },
    ///     Err(err) => {
    ///         eprintln!("Failed to create the interpreter: {}", err);
    ///         assert!(false);
    ///     },
    /// }
    /// ```
    ///
    /// This function initializes a new `Interpreter` instance by analyzing the provided TI-BASIC bytecode.
    /// It identifies and stores label positions and sets the initial byte pointer to 0.
    /// If any errors occur during the initialization process, they are wrapped in an `anyhow::Error` and returned.
    ///
    /// Labels are recognized in the bytecode by the `Lbl` marker (0xD6), followed by one or two alphanumeric
    /// characters, and terminated by a newline (0x3F). Labels are used to specify jump targets in the bytecode.
    ///
    /// The `Interpreter` struct provides methods for interpreting the bytecode and executing the program.
    ///
    /// # Error Handling
    ///
    /// This function returns a `Result` containing the initialized `Interpreter` if successful, or an error if
    /// any issues occur. If the provided bytecode is invalid or contains errors, an error will be returned,
    /// describing the issue.
    ///
    /// # Note
    ///
    /// This function does not perform actual bytecode interpretation. It focuses on the setup and preparation
    /// of the `Interpreter` for bytecode execution.
    pub fn new(ti_program: &TIFile) -> Result<Self, anyhow::Error> {
        let labels = find_labels(&ti_program.data)?;
        Ok(Self {
            bytes: ti_program.data.to_vec(),
            labels,
            bytes_pointer: 0,
        })
    }

    pub fn interpret_bytes(&mut self) {}

    pub fn interpret_next_byte(&mut self) {}
}
