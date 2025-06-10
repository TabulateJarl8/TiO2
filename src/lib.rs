/// The `translation` module provides functionality for translating between source code and bytecode
/// for TI-84 Plus calculators. It includes utilities for managing byte tokens, compiling source code
/// into bytecode, and decompiling bytecode into source code.
pub mod translation {
    pub mod common;
    pub mod compile;
    pub mod decompile;
    pub mod opcode;
    pub mod tokens;
}

pub mod errors;
#[cfg(feature = "interpreter")]
pub mod interpreter;
pub mod utils;
