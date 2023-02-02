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

    fn san_check_token_pos(
        &self,
        pos: usize,
        san_token_to_check: SanTokenKinds,
        sanjaiyan_token_collections: &mut Lexer<SanTokenKinds>,
    ) -> (bool, SanTokenKinds) {
        if let Some(sanjaiyan_pos_token) = sanjaiyan_token_collections.nth(pos) {
            if sanjaiyan_pos_token == san_token_to_check {
                (true, san_token_to_check)
            } else {
                (false, san_token_to_check)
            }
        } else {
            (false, san_token_to_check)
        }
    }

    pub fn sanjaiyan_organize_tokens(&self) -> Vec<SanTokenKinds> {
        let san_organized_tokens: Vec<SanTokenKinds> = Vec::new();

        let mut san_tokens_collection = self.san_tokenize();

        for (current_san_pos, current_san_token) in san_tokens_collection.clone().enumerate() {
            match current_san_token {
                SanTokenKinds::PascalProgramStart => todo!(),
                SanTokenKinds::PascalProgramEnd => todo!(),
                SanTokenKinds::PascalCodeBlockBegin => todo!(),
                SanTokenKinds::PascalCodeBlockEnd => todo!(),
                SanTokenKinds::LetDeclare => todo!(),
                SanTokenKinds::ConstDeclare => todo!(),
                SanTokenKinds::AssignVar => todo!(),
                SanTokenKinds::DataType(_) => todo!(),
                SanTokenKinds::LeftParen => todo!(),
                SanTokenKinds::RightParen => todo!(),
                SanTokenKinds::SemiColon => todo!(),
                SanTokenKinds::Colon => todo!(),
                SanTokenKinds::Comma => todo!(),
                SanTokenKinds::AndOp => todo!(),
                SanTokenKinds::OrOp => todo!(),
                SanTokenKinds::Indentifier(_) => todo!(),
                SanTokenKinds::Number(_) => todo!(),
                SanTokenKinds::Text(_) => todo!(),
                SanTokenKinds::SanError => todo!(),
            }
        }

        san_organized_tokens
    }
}
