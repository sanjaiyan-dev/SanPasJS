use std::{
    fs::{self, File},
    io::ErrorKind,
};

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
                // Special Characters
                SanTokenKinds::SemiColon => san_js_code.push(';'),
                SanTokenKinds::SanPascalNewLine => san_js_code.push('\n'),
                SanTokenKinds::LeftParen => san_js_code.push('('),
                SanTokenKinds::RightParen => san_js_code.push(')'),
                SanTokenKinds::LeftSqrParen => san_js_code.push('['),
                SanTokenKinds::RightSqrParen => san_js_code.push(']'),
                SanTokenKinds::Comma => san_js_code.push_str(", "),

                SanTokenKinds::PascalProgramStart | SanTokenKinds::ProcedureFunc => {
                    san_js_code.push_str("function ")
                }
                SanTokenKinds::PascalProgramEnd => san_js_code.push_str("};"),
                SanTokenKinds::PascalCodeBlockBegin => san_js_code.push('{'),
                SanTokenKinds::PascalCodeBlockEnd => san_js_code.push('}'),

                // Variables
                SanTokenKinds::LetDeclare => san_js_code.push_str("let "),
                SanTokenKinds::ConstDeclare => san_js_code.push_str("const "),
                SanTokenKinds::AssignVar => san_js_code.push_str(" = "),

                // Special Function
                SanTokenKinds::OutputWriteFunc => san_js_code.push_str("alert"),
                SanTokenKinds::InputReadFunc => san_js_code.push_str("prompt"),
                SanTokenKinds::HtmlOutputFunc => {
                    san_js_code.push_str("document.body.innerHTML += ");
                }
                SanTokenKinds::TextHtmlOutputFunc => {
                    san_js_code.push_str("document.body.innerText += ");
                }
                SanTokenKinds::HtmlOutputClearFunc => {
                    san_js_code.push_str("document.body.innerHTML = ''");
                }
                SanTokenKinds::SanPascalOutputFmt => {
                    san_js_code.push_str(".join('')");
                }

                // Operators
                SanTokenKinds::Plus => san_js_code.push('+'),
                SanTokenKinds::Minus => san_js_code.push('-'),
                SanTokenKinds::Multiply => san_js_code.push_str(" * "),
                SanTokenKinds::Divide => san_js_code.push_str(" / "),
                SanTokenKinds::Reminder => san_js_code.push_str(" % "),
                SanTokenKinds::Equal => san_js_code.push_str(" == "),
                SanTokenKinds::NotEqual => san_js_code.push_str(" != "),
                SanTokenKinds::Greater => san_js_code.push_str(" > "),
                SanTokenKinds::GreaterEqual => san_js_code.push_str(" >= "),
                SanTokenKinds::Less => san_js_code.push_str(" < "),
                SanTokenKinds::LessEqual => san_js_code.push_str(" <= "),
                SanTokenKinds::BitNo => san_js_code.push_str(" ~"),

                SanTokenKinds::AndOp => san_js_code.push_str(" && "),
                SanTokenKinds::OrOp => san_js_code.push_str(" || "),
                SanTokenKinds::NotOp => san_js_code.push_str(" !"),

                //Conditionals
                SanTokenKinds::IfStatement => san_js_code.push_str("if"),
                SanTokenKinds::ElseStatement => san_js_code.push_str("else "),

                // Loops
                SanTokenKinds::ForLoop => san_js_code.push_str("for"),
                SanTokenKinds::WhileLoop | SanTokenKinds::RepeatLoopUntil => {
                    san_js_code.push_str("while ");
                }
                SanTokenKinds::RepeatLoop => san_js_code.push_str("do"),

                // Ident, Strings, Numbers and Comments
                SanTokenKinds::Identifier(san_ident_name) => san_js_code.push_str(san_ident_name),
                SanTokenKinds::Text(san_text) => san_js_code.push_str(san_text),
                SanTokenKinds::Comment(san_comment) => {
                    san_js_code.push_str(&format!("/* {san_comment} */"))
                }
                SanTokenKinds::Number(san_num) => {
                    san_js_code.push_str(&format!("parseFloat({san_num})"))
                }
                SanTokenKinds::SanTrue => san_js_code.push_str("true"),
                SanTokenKinds::SanFalse => san_js_code.push_str("false"),
                SanTokenKinds::NullValue => san_js_code.push_str("null"),

                _ => {}
            }
        }

        san_js_code
    }

    pub fn sanjaiyan_write_to_js_file_san(&self, san_file_to_write: &str) {
        let sanjaiyan_js_code = self.sanjaiyan_parser();
        if let Err(san_err) = fs::write(san_file_to_write, sanjaiyan_js_code.as_bytes()) {
            if san_err.kind() == ErrorKind::NotFound {
                match File::create(san_file_to_write) {
                    Ok(..) => {
                        if let Err(..) = fs::write(san_file_to_write, sanjaiyan_js_code.as_bytes())
                        {
                            eprintln!("Oops! Failed to write the following Javascript code to the output file :(");
                            println!("\n{sanjaiyan_js_code}");
                        }
                    }
                    Err(..) => {
                        eprintln!("Oops! Failed to write the following Javascript code to the output file :(");
                        println!("\n{sanjaiyan_js_code}");
                    }
                }
            } else {
                eprintln!(
                    "Oops! Failed to write the following Javascript code to the output file :("
                );
                println!("\n{sanjaiyan_js_code}");
            }
        } else {
            println!(
                r#"Sucessfully compiled the Pascal file to the following Javascript file-: "{san_file_to_write}"."#
            );
        }
    }
}
