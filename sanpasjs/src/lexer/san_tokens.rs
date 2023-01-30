use logos::{Lexer, Logos};

fn san_to_text(san_lex: &mut Lexer<SanTokenKinds>) -> Option<String> {
    Some(san_lex.slice().to_string())
}

fn san_to_number(san_lex: &mut Lexer<SanTokenKinds>) -> Option<f64> {
    Some(san_lex.slice().parse().ok()?)
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum SanTokenKinds {
    #[token("program", ignore(ascii_case))]
    SanPascalProgramBegin,

    #[regex(r"[a-zA-Z_?]+", san_to_text)]
    SanIndentifier(String),

    #[regex(r"([0-9]+[.])?[0-9]+", san_to_number)]
    SanNumber(f64),

    #[regex(r##""(?:[^"\\]|\\.)*""##, san_to_text)]
    SanText(String),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    SanError,
}
