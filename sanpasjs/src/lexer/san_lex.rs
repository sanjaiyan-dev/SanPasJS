use logos::{Lexer, Logos};
use std::{fs, process};

use super::san_tokens::SanTokenKinds;

pub struct SanjaiyanPascalCode {
    pascal_program_code: String,
}

impl SanjaiyanPascalCode {
    pub fn new(san_file_name: &str) -> Self {
        if let Ok(san_pascal_program) = fs::read_to_string(san_file_name) {
            SanjaiyanPascalCode {
                pascal_program_code: san_pascal_program,
            }
        } else {
            eprintln!("Oops failed to read the pascal file named '{san_file_name}' :( ");
            process::exit(1);
        }
    }

    fn san_tokenize(&self) -> Lexer<SanTokenKinds> {
        SanTokenKinds::lexer(&self.pascal_program_code)
    }

    fn san_check_token_pos(
        &self,
        sanjaiyan_tokens_array: &[SanTokenKinds],
        san_position_to_check: usize,
        san_token_to_find: SanTokenKinds,
    ) -> (bool, SanTokenKinds) {
        if let Some(san_token_to_compare) = sanjaiyan_tokens_array.get(san_position_to_check) {
            if san_token_to_compare == &san_token_to_find {
                (true, san_token_to_find)
            } else {
                (false, san_token_to_compare.to_owned())
            }
        } else {
            (false, SanTokenKinds::NullValue)
        }
    }

    pub fn sanjaiyan_organize_tokens(&self) -> Vec<SanTokenKinds> {
        let mut san_organized_tokens = Vec::new();
        let sanjaiyan_token_array_format = self.san_tokenize().collect::<Vec<_>>();

        let mut sanjaiyan_firtst_semicolon_came = false;
        let mut sanjaiyan_current_output_func = false;

        for (san_current_pos, san_current_token) in sanjaiyan_token_array_format.iter().enumerate()
        {
            match san_current_token {
                SanTokenKinds::PascalCodeBlockBegin => {
                    san_organized_tokens.push(SanTokenKinds::PascalCodeBlockBegin);
                    san_organized_tokens.push(SanTokenKinds::SanPascalNewLine);
                }
                SanTokenKinds::PascalProgramEnd => {
                    san_organized_tokens.push(SanTokenKinds::PascalProgramEnd);
                    san_organized_tokens.push(SanTokenKinds::SanPascalNewLine);
                    san_organized_tokens.push(SanTokenKinds::Comment(
                        "Developed using 'SanPasJs' 🌟✨🌟".to_string(),
                    ));
                    san_organized_tokens.push(SanTokenKinds::SanPascalNewLine);
                }
                SanTokenKinds::PascalCodeBlockEnd => {
                    san_organized_tokens.push(SanTokenKinds::PascalCodeBlockEnd);
                }

                SanTokenKinds::ProcedureFunc => {
                    san_organized_tokens.push(SanTokenKinds::ProcedureFunc);
                }
                // Variables related
                SanTokenKinds::LetDeclare => {
                    let san_num_to_check_before = {
                        if san_current_pos as isize - 3 <= 0 {
                            0
                        } else {
                            san_current_pos - 3
                        }
                    };
                    let (san_check_procedure_before, ..) = self.san_check_token_pos(
                        &sanjaiyan_token_array_format,
                        san_num_to_check_before,
                        SanTokenKinds::ProcedureFunc,
                    );
                    if !san_check_procedure_before {
                        san_organized_tokens.push(SanTokenKinds::LetDeclare);
                    }
                }
                SanTokenKinds::ConstDeclare => {
                    san_organized_tokens.push(SanTokenKinds::ConstDeclare);
                }
                SanTokenKinds::AssignVar => san_organized_tokens.push(SanTokenKinds::AssignVar),
                SanTokenKinds::DataTypeString => {
                    let (san_check_next_semi_colon, ..) = self.san_check_token_pos(
                        &sanjaiyan_token_array_format,
                        san_current_pos + 1,
                        SanTokenKinds::SemiColon,
                    );
                    if san_check_next_semi_colon {
                        san_organized_tokens.push(SanTokenKinds::AssignVar);
                        san_organized_tokens.push(SanTokenKinds::Text(r###"" ""###.to_string()));
                    }
                }
                SanTokenKinds::DataTypeNumber => {
                    let (san_check_next_semi_colon, ..) = self.san_check_token_pos(
                        &sanjaiyan_token_array_format,
                        san_current_pos + 1,
                        SanTokenKinds::SemiColon,
                    );

                    if san_check_next_semi_colon {
                        san_organized_tokens.push(SanTokenKinds::AssignVar);
                        san_organized_tokens.push(SanTokenKinds::Number(0.00));
                    }
                }
                SanTokenKinds::DataTypeBoolean => {
                    let (san_check_next_semi_colon, ..) = self.san_check_token_pos(
                        &sanjaiyan_token_array_format,
                        san_current_pos + 1,
                        SanTokenKinds::SemiColon,
                    );

                    if san_check_next_semi_colon {
                        san_organized_tokens.push(SanTokenKinds::AssignVar);
                        san_organized_tokens.push(SanTokenKinds::NullValue);
                    }
                }
                SanTokenKinds::DataTypeArray => {
                    san_organized_tokens.push(SanTokenKinds::AssignVar);
                    san_organized_tokens.push(SanTokenKinds::LeftSqrParen);
                    san_organized_tokens.push(SanTokenKinds::RightSqrParen);
                    san_organized_tokens.push(SanTokenKinds::SemiColon);
                    san_organized_tokens.push(SanTokenKinds::SanPascalNewLine);
                }
                //Special Functions
                SanTokenKinds::OutputWriteFunc => {
                    sanjaiyan_current_output_func = true;
                    san_organized_tokens.push(SanTokenKinds::OutputWriteFunc);
                }
                SanTokenKinds::HtmlOutputFunc => {
                    san_organized_tokens.push(SanTokenKinds::HtmlOutputFunc);
                }
                SanTokenKinds::HtmlOutputClearFunc => {
                    san_organized_tokens.push(SanTokenKinds::HtmlOutputClearFunc);
                }
                SanTokenKinds::TextHtmlOutputFunc => {
                    san_organized_tokens.push(SanTokenKinds::TextHtmlOutputFunc);
                }
                SanTokenKinds::InputReadFunc => {
                    let san_get_ident = sanjaiyan_token_array_format
                        .get(san_current_pos + 2)
                        .unwrap_or(&SanTokenKinds::NullValue);

                    if let SanTokenKinds::Identifier(san_input_ident_name) = san_get_ident {
                        san_organized_tokens
                            .push(SanTokenKinds::Identifier(san_input_ident_name.to_string()));
                        san_organized_tokens.push(SanTokenKinds::AssignVar);
                        san_organized_tokens.push(SanTokenKinds::InputReadFunc);
                    }
                }

                // If - else Statements
                SanTokenKinds::IfStatement => {
                    san_organized_tokens.push(SanTokenKinds::IfStatement);
                    san_organized_tokens.push(SanTokenKinds::LeftParen);
                }
                SanTokenKinds::IfStatementCodeBlock => {
                    san_organized_tokens.push(SanTokenKinds::RightParen);
                }
                SanTokenKinds::ElseStatement => {
                    let (san_check_before_semicolon, ..) = self.san_check_token_pos(
                        &sanjaiyan_token_array_format,
                        san_current_pos - 1,
                        SanTokenKinds::SemiColon,
                    );
                    if san_check_before_semicolon {
                        san_organized_tokens.push(SanTokenKinds::ElseStatement);
                    } else {
                        san_organized_tokens.push(SanTokenKinds::SemiColon);
                        san_organized_tokens.push(SanTokenKinds::SanPascalNewLine);
                        san_organized_tokens.push(SanTokenKinds::ElseStatement);
                    }
                }

                // Loops Statements
                SanTokenKinds::ForLoop => {
                    san_organized_tokens.push(SanTokenKinds::ForLoop);
                    san_organized_tokens.push(SanTokenKinds::LeftParen);
                }
                SanTokenKinds::LoopStatementCodeBlock => {
                    san_organized_tokens.push(SanTokenKinds::RightParen);
                }
                SanTokenKinds::ForLoopIncrease => {
                    let san_for_loop_target_var_name = sanjaiyan_token_array_format
                        .get(san_current_pos - 3)
                        .unwrap_or(&SanTokenKinds::NullValue);
                    let san_for_loop_target_value = sanjaiyan_token_array_format
                        .get(san_current_pos + 1)
                        .unwrap_or(&SanTokenKinds::NullValue);

                    san_organized_tokens.push(SanTokenKinds::SemiColon);
                    if let (
                        SanTokenKinds::Identifier(sanjaiyan_loop_var),
                        SanTokenKinds::Number(san_target_num),
                    ) = (san_for_loop_target_var_name, san_for_loop_target_value)
                    {
                        san_organized_tokens
                            .push(SanTokenKinds::Identifier(sanjaiyan_loop_var.to_string()));
                        san_organized_tokens.push(SanTokenKinds::LessEqual);
                        san_organized_tokens.push(SanTokenKinds::Number(*san_target_num));
                    }
                    san_organized_tokens.push(SanTokenKinds::SemiColon);
                }

                SanTokenKinds::ForLoopDecrease => {
                    let san_for_loop_target_var_name = sanjaiyan_token_array_format
                        .get(san_current_pos - 3)
                        .unwrap_or(&SanTokenKinds::NullValue);
                    let san_for_loop_target_value = sanjaiyan_token_array_format
                        .get(san_current_pos + 1)
                        .unwrap_or(&SanTokenKinds::NullValue);

                    san_organized_tokens.push(SanTokenKinds::SemiColon);
                    if let (
                        SanTokenKinds::Identifier(sanjaiyan_loop_var),
                        SanTokenKinds::Number(san_target_num),
                    ) = (san_for_loop_target_var_name, san_for_loop_target_value)
                    {
                        san_organized_tokens
                            .push(SanTokenKinds::Identifier(sanjaiyan_loop_var.to_string()));
                        san_organized_tokens.push(SanTokenKinds::GreaterEqual);
                        san_organized_tokens.push(SanTokenKinds::Number(*san_target_num));
                    }
                    san_organized_tokens.push(SanTokenKinds::SemiColon);
                }

                //Operators
                SanTokenKinds::Plus => san_organized_tokens.push(SanTokenKinds::Plus),
                SanTokenKinds::Minus => san_organized_tokens.push(SanTokenKinds::Minus),
                SanTokenKinds::Multiply => san_organized_tokens.push(SanTokenKinds::Multiply),
                SanTokenKinds::Divide => san_organized_tokens.push(SanTokenKinds::Divide),
                SanTokenKinds::Reminder => san_organized_tokens.push(SanTokenKinds::Reminder),
                SanTokenKinds::Equal => {
                    let san_get_ident = sanjaiyan_token_array_format
                        .get(san_current_pos - 1)
                        .unwrap_or(&SanTokenKinds::NullValue);

                    match san_get_ident {
                        SanTokenKinds::DataTypeString
                        | SanTokenKinds::DataTypeNumber
                        | SanTokenKinds::DataTypeBoolean
                        | SanTokenKinds::DataTypeArray => {
                            san_organized_tokens.push(SanTokenKinds::AssignVar);
                        }
                        _ => {
                            san_organized_tokens.push(SanTokenKinds::Equal);
                        }
                    }
                }
                SanTokenKinds::NotEqual => san_organized_tokens.push(SanTokenKinds::NotEqual),
                SanTokenKinds::Greater => san_organized_tokens.push(SanTokenKinds::Greater),
                SanTokenKinds::GreaterEqual => {
                    san_organized_tokens.push(SanTokenKinds::GreaterEqual);
                }
                SanTokenKinds::Less => san_organized_tokens.push(SanTokenKinds::Less),
                SanTokenKinds::LessEqual => san_organized_tokens.push(SanTokenKinds::LessEqual),
                SanTokenKinds::BitNo => san_organized_tokens.push(SanTokenKinds::BitNo),

                SanTokenKinds::AndOp => san_organized_tokens.push(SanTokenKinds::AndOp),
                SanTokenKinds::OrOp => san_organized_tokens.push(SanTokenKinds::OrOp),
                SanTokenKinds::NotOp => san_organized_tokens.push(SanTokenKinds::NotOp),

                // Special Characters
                SanTokenKinds::SemiColon => {
                    let (san_next_begin_statement, ..) = self.san_check_token_pos(
                        &sanjaiyan_token_array_format,
                        san_current_pos + 1,
                        SanTokenKinds::PascalCodeBlockBegin,
                    );
                    if sanjaiyan_firtst_semicolon_came {
                        if !san_next_begin_statement {
                            san_organized_tokens.push(SanTokenKinds::SemiColon);
                            san_organized_tokens.push(SanTokenKinds::SanPascalNewLine);
                        }
                    } else {
                        san_organized_tokens.push(SanTokenKinds::SanPascalNewLine);
                        sanjaiyan_firtst_semicolon_came = true;
                    }
                }
                SanTokenKinds::LeftParen => {
                    san_organized_tokens.push(SanTokenKinds::LeftParen);
                    let (san_check_before_output_func, ..) = self.san_check_token_pos(
                        &sanjaiyan_token_array_format,
                        san_current_pos - 1,
                        SanTokenKinds::OutputWriteFunc,
                    );
                    if san_check_before_output_func {
                        san_organized_tokens.push(SanTokenKinds::LeftSqrParen);
                    }
                }
                SanTokenKinds::RightParen => {
                    if sanjaiyan_current_output_func {
                        san_organized_tokens.push(SanTokenKinds::RightSqrParen);
                        sanjaiyan_current_output_func = false;
                    }
                    san_organized_tokens.push(SanTokenKinds::RightParen);
                }
                SanTokenKinds::LeftSqrParen => {
                    san_organized_tokens.push(SanTokenKinds::LeftSqrParen);
                }
                SanTokenKinds::RightSqrParen => {
                    san_organized_tokens.push(SanTokenKinds::RightSqrParen);
                }
                SanTokenKinds::Comma => san_organized_tokens.push(SanTokenKinds::Comma),

                SanTokenKinds::NullValue => san_organized_tokens.push(SanTokenKinds::NullValue),
                SanTokenKinds::SanTrue => san_organized_tokens.push(SanTokenKinds::SanTrue),
                SanTokenKinds::SanFalse => san_organized_tokens.push(SanTokenKinds::SanFalse),

                // Numbers and Strings
                SanTokenKinds::Identifier(san_ident_name) => {
                    san_organized_tokens.push(SanTokenKinds::Identifier(san_ident_name.to_string()))
                }
                SanTokenKinds::Text(sanjaiyan_string_txt) => {
                    san_organized_tokens
                        .push(SanTokenKinds::Text(sanjaiyan_string_txt.to_string()));
                }
                SanTokenKinds::Number(sanjaiyan_number) => {
                    let (san_check_next_forloopdo, ..) = self.san_check_token_pos(
                        &sanjaiyan_token_array_format,
                        san_current_pos + 1,
                        SanTokenKinds::LoopStatementCodeBlock,
                    );

                    if san_check_next_forloopdo {
                        let san_target_var_for_loop = sanjaiyan_token_array_format
                            .get(san_current_pos - 4)
                            .unwrap_or(&SanTokenKinds::NullValue);

                        if let SanTokenKinds::Identifier(sanjaiyan_forloop_ident) =
                            san_target_var_for_loop
                        {
                            san_organized_tokens.push(SanTokenKinds::Identifier(
                                sanjaiyan_forloop_ident.to_string(),
                            ));
                            let san_for_loop_type = sanjaiyan_token_array_format
                                .get(san_current_pos - 1)
                                .unwrap_or(&SanTokenKinds::NullValue);
                            if san_for_loop_type == &SanTokenKinds::ForLoopIncrease {
                                san_organized_tokens.push(SanTokenKinds::Plus);
                                san_organized_tokens.push(SanTokenKinds::Plus);
                            }
                            if san_for_loop_type == &SanTokenKinds::ForLoopDecrease {
                                san_organized_tokens.push(SanTokenKinds::Minus);
                                san_organized_tokens.push(SanTokenKinds::Minus);
                            }
                        } else {
                            san_organized_tokens.push(SanTokenKinds::Number(*sanjaiyan_number));
                        }
                    } else {
                        san_organized_tokens.push(SanTokenKinds::Number(*sanjaiyan_number));
                    }
                }
                SanTokenKinds::Comment(sanjaiyan_comment) => {
                    san_organized_tokens
                        .push(SanTokenKinds::Comment(sanjaiyan_comment.to_string()));
                    san_organized_tokens.push(SanTokenKinds::SanPascalNewLine);
                }

                san_missed_tokens => {
                    if san_missed_tokens == &SanTokenKinds::ERROR {
                        san_organized_tokens.push(SanTokenKinds::Comment("Error".to_string()));
                    }
                }
            }
        }

        println!("{san_organized_tokens:?} \n");

        san_organized_tokens
    }
}
