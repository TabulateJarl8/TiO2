use crate::translation::{common::TIFile, tokens::{Byte, BYTE_TOKENS}};

#[derive(Debug)]
enum TIToken {
    Number(f64),
    String(String),
    Function(u8),
    Expression(Vec<u8>),
}

#[derive(Debug)]
struct TIProgramState {
    tokens: Vec<TIToken>,
}

impl TIProgramState {
    fn new() -> Self {
        Self {
            tokens: Vec::new(),
        }
    }

    fn add_string(&mut self, token: String) {
        self.tokens.push(TIToken::String(token));
    }

    fn add_function(&mut self, token: u8) {
        self.tokens.push(TIToken::Function(token));
    }

    fn add_number(&mut self, token: f64) {
        self.tokens.push(TIToken::Number(token));
    }
}

pub fn tokenize_bytecode(ti_program: TIFile) {
    let bytecode = ti_program.data;
    println!("{:x?}", &bytecode);
    let mut bytecode_pc: usize = 0;
    let mut consuming_string = false;
    let mut consuming_number = false;
    let mut string_buffer = String::new();
    let mut number_buffer = String::new();

    let mut program_state = TIProgramState::new();

    while bytecode_pc < bytecode.len() {
        let current_token = bytecode[bytecode_pc];
        println!("{:x?}", current_token);

        
        if consuming_string {
            if current_token == 0x2A || current_token == 0x3F {
                consuming_string = false;
                program_state.add_string(string_buffer.clone());
                string_buffer.clear();
            } else {
                string_buffer += BYTE_TOKENS.get(&Byte::Single(current_token)).unwrap();
            }
        } else if consuming_number {
            if (0x30..=0x39).contains(&current_token) || [0x3A, 0xB0].contains(&current_token) {
                number_buffer += BYTE_TOKENS.get(&Byte::Single(current_token)).unwrap();
            } else {
                consuming_number = false;
                program_state.add_number(number_buffer.parse::<f64>().expect("number failed to convert from string"));
                number_buffer.clear();

                // TODO: this is terrible and the next token just isnt being included
            }
        } else {
            if current_token == 0x2A {
                consuming_string = true;
            } else if (0x30..=0x39).contains(&current_token) || [0x3A, 0xB0].contains(&current_token) {
                consuming_number = true;
                number_buffer += BYTE_TOKENS.get(&Byte::Single(current_token)).unwrap();
            } else {
                program_state.add_function(current_token);
            }
        }
        bytecode_pc += 1;
    }

    dbg!(program_state);

}