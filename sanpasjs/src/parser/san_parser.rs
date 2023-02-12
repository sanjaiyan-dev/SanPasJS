use std::fs;

use crate::lexer::san_tokens::SanTokenKinds;

pub struct SanjaiyanPascalTokens {
    sanjaiyan_pas_tokens: Vec<SanTokenKinds>,
}

impl SanjaiyanPascalTokens {
    pub fn new(sanjaiyan_pascal_tokens_san: Vec<SanTokenKinds>) -> Self {
        SanjaiyanPascalTokens {
            sanjaiyan_pas_tokens: sanjaiyan_pascal_tokens_san,
        }
    }

    fn sanjaiyan_parser(&self) -> String {
        let mut san_js_code = String::new();
        for san_current_token in &self.sanjaiyan_pas_tokens {
            match san_current_token {
                SanTokenKinds::PascalProgramStart => san_js_code.push_str("function "),
                _ => {
                    todo!();
                }
            }
        }

        san_js_code
    }

    pub fn sanjaiyan_write_to_js_file_san(&self, san_file_to_write: &str) {
        let sanjaiyan_js_code = self.sanjaiyan_parser();
        if let Err(..) = fs::write(san_file_to_write, sanjaiyan_js_code.as_bytes()) {
            println!("Oops! Failed to write the following Javascript code to the output file :(");
            println!("\n {sanjaiyan_js_code}");
        } else {
            println!(
                r#"Sucessfully compiled the Pascal file to the following Javascript file-: "{san_file_to_write}"."#
            );
        }
    }
}
