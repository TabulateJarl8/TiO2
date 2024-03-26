use crate::translation::{
    common::TIFile,
    tokens::{Byte, BYTE_TOKENS},
};

#[derive(Debug)]
enum TIToken {
    Number(f64),
    String(String),
    Function(u8),
    Expression(Vec<u8>),
}

/// A struct representing the state of a TI program
#[derive(Debug)]
struct TIProgramState {
    tokens: Vec<TIToken>,
}

impl TIProgramState {
    fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    /// Add a string token to the token list
    fn add_string(&mut self, token: String) {
        self.tokens.push(TIToken::String(token));
    }

    /// Add a function token to the token list
    fn add_function(&mut self, token: u8) {
        self.tokens.push(TIToken::Function(token));
    }

    /// Add a number token to the token list
    fn add_number(&mut self, token: f64) {
        self.tokens.push(TIToken::Number(token));
    }
}

pub fn tokenize_bytecode(ti_program: TIFile) {
    let bytecode = ti_program.data;
    println!("{:x?}", &bytecode);
    let mut bytecode_pc: usize = 0;

    let mut program_state = TIProgramState::new();

    while bytecode_pc < bytecode.len() {
        let current_token = bytecode[bytecode_pc];

        match current_token {
            // check if the current token is `"` so that we can start tokenizing
            // a string
            0x2A => {
                // advance past the current " since we don't tokenize that
                bytecode_pc += 1;
                consume_string(&mut bytecode_pc, &bytecode, &mut program_state);
            }
            // check if the current token is numerical, a `.`, or the negative sign
            // if so, we parse a number
            0x30..=0x39 | 0x3A | 0xB0 => {
                consume_number(&mut bytecode_pc, &bytecode, &mut program_state)
            }
            _ => program_state.add_function(current_token),
        }
        bytecode_pc += 1;
    }

    println!("{:x?}", program_state);
}

/// Try to consume a string from the current set of tokens
fn consume_string(bytecode_pc: &mut usize, bytecode: &[u8], program_state: &mut TIProgramState) {
    let mut string_buffer = String::new();
    let mut consuming_string = true;

    // loop until the string tokenization is complete
    while *bytecode_pc < bytecode.len() && consuming_string {
        let current_token = bytecode[*bytecode_pc];
        // check if we're at the end of a string (`"` or `\n` is encountered)
        if current_token == 0x2A || current_token == 0x3F {
            consuming_string = false;
            program_state.add_string(string_buffer.clone());

            // we dont increment the pc here because its incremented in tokenize_bytecode
            // after the function returns
        } else {
            // we're still in the string, so add it to the buffer and continue
            string_buffer += BYTE_TOKENS.get(&Byte::Single(current_token)).unwrap();
            *bytecode_pc += 1;
        }
    }
}

/// Try to consume a number from the current set of tokens
fn consume_number(bytecode_pc: &mut usize, bytecode: &[u8], program_state: &mut TIProgramState) {
    let mut consuming_number = true;
    let mut number_buffer = String::new();

    // loop until the number tokenization is complete
    while *bytecode_pc < bytecode.len() && consuming_number {
        let current_token = bytecode[*bytecode_pc];

        // check if the current token is still part of a number
        if (0x30..=0x39).contains(&current_token) || [0x3A, 0xB0].contains(&current_token) {
            // we're still in a number so we can add it to the buffer and continue
            number_buffer += BYTE_TOKENS
                .get(&Byte::Single(current_token))
                .expect("numerical tokens should be available in token map");
            *bytecode_pc += 1;
        } else {
            // we're done with the number, parse the string as a f64 and add it as a token
            consuming_number = false;
            program_state.add_number(
                number_buffer
                    .parse::<f64>()
                    .expect("number failed to convert from string"),
            );

            // we decrease this by one 1 since the outer function increases it
            // and we haven't done anything with the current token
            *bytecode_pc -= 1;
        }
    }
}
