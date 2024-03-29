use std::{collections::HashMap, io::Write};

use crate::{
    translation::{
        common::TIFile,
        tokens::{Byte, BYTE_TOKENS},
    },
    utils,
};

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
    /// A buffer string for consuming tokens
    pub current_token_consumer: String,
    pub variables: HashMap<String, String>,
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
    /// use tio2::{interpreter::Interpreter, translation::common::TIFile};
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
    ///     }
    ///     Err(err) => {
    ///         eprintln!("Failed to create the interpreter: {}", err);
    ///         assert!(false);
    ///     }
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
            current_token_consumer: String::new(),
            variables: HashMap::new(),
        })
    }

    /// Parses the TI-BASIC bytecode in the `bytes` field of the `Interpreter` and populates the `instruction_stack`.
    ///
    /// This function iterates over the bytecode, parsing and categorizing the tokens based on the byte values.
    /// It recognizes various types of instructions, including RHS functions, LHS functions, functions with no arguments,
    /// functions with arguments on both sides, and conditional instructions. The parsed tokens are pushed onto the
    /// `instruction_stack`.
    ///
    /// # Errors
    ///
    /// If an unexpected byte value is encountered or if the bytecode is invalid, this function may return an error
    /// describing the issue.
    ///
    /// # Example
    ///
    /// ```
    /// use tio2::{
    ///     interpreter::Interpreter,
    ///     translation::{common::TIFile, tokens::TokenType},
    /// };
    ///
    /// // Equal to `Disp "A"`
    /// let ti_program = TIFile {
    ///     header: [0; 74],
    ///     data: vec![0xde, 0x2a, 0x41, 0x2a],
    ///     footer: vec![],
    /// };
    /// let mut interpreter = Interpreter::new(&ti_program).expect("Failed to create interpreter");
    /// interpreter.parse_bytes().expect("Failed to parse bytes");
    /// // The instruction stack is now populated with parsed tokens.
    /// assert_eq!(
    ///     interpreter.instruction_stack,
    ///     vec![
    ///         TokenType::RHSFunction("Disp "),
    ///         TokenType::Token("\"A\"".into())
    ///     ],
    /// );
    /// ```
    pub fn parse_bytes(&mut self) -> Result<(), anyhow::Error> {
        

        Ok(())
    }
}
