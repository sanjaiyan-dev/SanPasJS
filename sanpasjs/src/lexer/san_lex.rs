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
        match sanjaiyan_token_collections.nth(pos) {
            Some(sanjaiyan_pos_token) => {
                if sanjaiyan_pos_token == san_token_to_check {
                    (true, san_token_to_check)
                } else {
                    (false, san_token_to_check)
                }
            }
            None => (false, san_token_to_check),
        }
    }

    pub fn sanjaiyan_organize_tokens(&self) -> Vec<SanTokenKinds> {
        let mut san_organized_tokens: Vec<SanTokenKinds> = Vec::new();

        let mut san_tokens_collection = self.san_tokenize();

        for (current_san_pos, current_san_token) in san_tokens_collection.clone().enumerate() {
            match current_san_token {
                SanTokenKinds::PascalCodeBlockBegin => {
                    if san_organized_tokens.contains(&SanTokenKinds::PascalCodeMainStart) {
                        san_organized_tokens.push(SanTokenKinds::PascalCodeMainStart);
                    } else {
                        san_organized_tokens.push(SanTokenKinds::PascalCodeBlockBegin);
                    }
                }
                SanTokenKinds::LetDeclare => san_organized_tokens.push(SanTokenKinds::LetDeclare),
                SanTokenKinds::Indentifier(san_current_ident) => {
                    san_organized_tokens.push(SanTokenKinds::Indentifier(san_current_ident))
                }
                SanTokenKinds::DataType(san_data_type) => {
                    let (san_check_semicolon_next, ..) = self.san_check_token_pos(
                        current_san_pos + 1,
                        SanTokenKinds::SemiColon,
                        &mut san_tokens_collection,
                    );
                    if san_check_semicolon_next {
                        match san_data_type.to_lowercase().as_str() {
                            "string" => {
                                san_organized_tokens.push(SanTokenKinds::AssignVar);
                                san_organized_tokens.push(SanTokenKinds::Text(" ".to_string()));
                            }
                            "integer" | "real" | "number" => {
                                san_organized_tokens.push(SanTokenKinds::AssignVar);
                                san_organized_tokens.push(SanTokenKinds::Number(0.00));
                            }
                            "char" => {
                                san_organized_tokens.push(SanTokenKinds::AssignVar);
                                san_organized_tokens.push(SanTokenKinds::Text(' '.to_string()));
                            }
                            _ => {
                                san_organized_tokens.push(SanTokenKinds::AssignVar);
                                san_organized_tokens.push(SanTokenKinds::NullValue);
                            }
                        }
                    }
                }

                _ => {
                    print!("Came ")
                }
            }
        }

        println!("{:?}", san_organized_tokens);

        san_organized_tokens
    }
}
