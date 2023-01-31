use logos::{Lexer, Logos};

fn san_to_text(san_lex: &mut Lexer<SanTokenKinds>) -> Option<String> {
    Some(san_lex.slice().to_string())
}

fn san_to_number(san_lex: &mut Lexer<SanTokenKinds>) -> Option<f64> {
    san_lex.slice().parse::<f64>().ok()
}

fn san_data_type(san_lex: &mut Lexer<SanTokenKinds>) -> Option<String> {
    if let Ok(mut san_data_type_str) = san_lex.slice().parse::<String>() {
        san_data_type_str.replace_range(0..1, "");
        Some(san_data_type_str)
    } else {
        None
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum SanTokenKinds {
    #[regex(r"program .*;", ignore(ascii_case))]
    PascalProgramStart,
    #[token("end.", ignore(ascii_case))]
    PascalProgramEnd,
    #[token("end", ignore(ascii_case))]
    EndOfTheTask,

    #[token("var", ignore(ascii_case))]
    LetDeclare,
    #[token("const", ignore(ascii_case))]
    ConstDeclare,
    #[token(":=", ignore(ascii_case))]
    AssignVar,

    #[token("(", ignore(ascii_case))]
    LeftParen,
    #[token(")", ignore(ascii_case))]
    RightParen,
    #[token(";", ignore(ascii_case))]
    SemiColon,
    #[token(":", ignore(ascii_case))]
    Colon,

    #[token("and", ignore(ascii_case))]
    AndOp,
    #[token("or", ignore(ascii_case))]
    OrOp,

    #[regex(r"[a-zA-Z_?]+", san_to_text)]
    Indentifier(String),

    #[regex(r"([0-9]+[.])?[0-9]+", san_to_number)]
    Number(f64),

    #[regex(r##""(?:[^"\\]|\\.)*""##, san_to_text)]
    Text(String),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    SanError,
}
