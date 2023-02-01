use logos::{Lexer, Logos};
use std::{fs, process};

use super::san_tokens::SanTokenKinds;

pub struct SanjaiyanPascalCode {
    pascal_file_path: String,
    pascal_program_code: String,
}

impl SanjaiyanPascalCode {
    pub fn new(san_file_name: String) -> Self {
        if let Ok(san_pascal_program) = fs::read_to_string(&san_file_name) {
            SanjaiyanPascalCode {
                pascal_file_path: san_file_name,
                pascal_program_code: san_pascal_program,
            }
        } else {
            eprintln!(
                "Oops failed to read the pascal file named '{}' :( ",
                san_file_name
            );
            process::exit(1);
        }
    }

    fn san_tokenize(&self) -> Lexer<SanTokenKinds> {
        SanTokenKinds::lexer(&self.pascal_program_code)
    }

    pub fn sanjaiyan_organize_tokens(&self) -> Vec<SanTokenKinds> {
        let san_organized_tokens: Vec<SanTokenKinds> = Vec::new();

        let san_tokens_collection = self.san_tokenize().enumerate();

        for san_token in san_tokens_collection {
            println!("{:?}", san_token.1)
        }

        san_organized_tokens
    }
}
