use crate::translation::{
    common::TIFile,
    tokens::{Byte, BYTE_TOKENS},
};

/// A function object
#[derive(Debug, Clone, PartialEq)]
struct Function {
    /// The opcode of the function. Can be one or two bytes.
    opcode: Byte,
    /// The arguments of the function
    args: Vec<TIToken>,
    /// Whether or not the function is a block function (i.e., it doesnt begin with `(`
    /// and you can only have 1 per line)
    block_function: bool,
    /// Internal value for keeping track of when to close the function. A value
    /// of 0 means that the function should be closed.
    open_parenthesis: u16,
}

impl Function {
    fn new(opcode: Byte, block_function: bool) -> Self {
        Self {
            opcode,
            args: Vec::new(),
            block_function,
            open_parenthesis: 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum TIToken {
    Number(f64),
    String(String),
    Function(Function),
    Expression(Vec<u8>),
    Token(Byte),
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

    fn push_token(&mut self, token: TIToken) {
        // check if the token should be added as a argument to the last function
        // or not
        if let Some(prev_tok) = self.tokens.last_mut() {
            if let TIToken::Function(func) = prev_tok {
                // newline, collapse function parenthesis and ignore
                if token == TIToken::Token(Byte::Single(0x3F)) {
                    func.open_parenthesis = 0;
                    return;
                // closing parenthesis
                } else if token == TIToken::Token(Byte::Single(0x11)) {
                    func.open_parenthesis -= 1;
                    return;
                }

                // check if the previous function is still open
                if func.open_parenthesis > 0 {
                    func.args.push(token);
                } else {
                    self.tokens.push(token);
                }
            } else {
                // last token isn't a function
                self.tokens.push(token);
            }
        } else {
            // the list is empty, push the tokens as normal
            self.tokens.push(token);
        }
    }

    /// Add a string token to the token list
    fn add_string(&mut self, token: String) {
        self.push_token(TIToken::String(token));
    }

    /// Add a function token to the token list
    fn add_function(&mut self, opcode: Byte, block_function: bool) {
        self.push_token(TIToken::Function(Function::new(opcode, block_function)));
    }

    /// Add a number token to the token list
    fn add_number(&mut self, token: f64) {
        self.push_token(TIToken::Number(token));
    }

    fn add_token(&mut self, token: Byte) {
        self.push_token(TIToken::Token(token));
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
            0x5C..=0x5E | 0x60..=0x63 | 0xAA | 0xBB | 0xEF | 0x7E => {
                // double byte tokens
                bytecode_pc += 1;
                let current_token = bytecode[bytecode_pc];

                match current_token {
                    _ => program_state.add_token(Byte::Single(current_token)),
                }
            }
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
            0x12..=0x28
            | 0x93
            | 0x9C
            | 0x9E..=0xA5
            | 0xA7
            | 0xB1..=0xBA
            | 0xBC..=0xCD
            | 0xDA..=0xDB
            | 0xE0
            | 0xE2..=0xE4
            | 0xE6..=0xE8
            | 0xEC..=0xEE => {
                // check for multi-line functions (can be started and ended with `(` and `)`)
                program_state.add_function(Byte::Single(current_token), false);
            }
            0x2E..=0x2F
            | 0x5F..=0x69
            | 0x73..=0x7D
            | 0x84..=0x92
            | 0x96..=0x9B
            | 0x9D
            | 0xA6
            | 0xA8..=0xA9
            | 0xAB
            | 0xAD
            | 0xCE..=0xD9
            | 0xDC..=0xDF
            | 0xE1
            | 0xE5
            | 0xE9..=0xEA
            | 0xF2..=0xFF => {
                // block level functions (only one per line, ended with `\n`)
                program_state.add_function(Byte::Single(current_token), true);
            }
            _ => program_state.add_token(Byte::Single(current_token)),
        }
        bytecode_pc += 1;
    }

    println!("{:#x?}", program_state);
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
