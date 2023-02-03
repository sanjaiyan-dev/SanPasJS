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
        san_data_type_str.remove(san_data_type_str.len() - 1);
        Some(san_data_type_str)
    } else {
        None
    }
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum SanTokenKinds {
    PascalCodeMainStart,

    #[regex(r"program.*;", ignore(ascii_case))]
    PascalProgramStart,
    #[token("end.", ignore(ascii_case))]
    PascalProgramEnd,
    #[token("begin", ignore(ascii_case))]
    PascalCodeBlockBegin,
    #[token("end", ignore(ascii_case))]
    PascalCodeBlockEnd,
    #[token("procedure", ignore(ascii_case))]
    ProcedureFunc,
    #[regex(r###"\{.*\.*}"###, san_to_text)]
    Comment(String),

    #[token("var", ignore(ascii_case))]
    LetDeclare,
    #[token("const", ignore(ascii_case))]
    ConstDeclare,
    #[token(":=", ignore(ascii_case))]
    AssignVar,

    #[regex(r":\s*[a-z].*(;|=)", san_data_type)]
    DataType(String),

    #[token("(", ignore(ascii_case))]
    LeftParen,
    #[token(")", ignore(ascii_case))]
    RightParen,
    #[token(";", ignore(ascii_case))]
    SemiColon,
    #[token(":", ignore(ascii_case))]
    Colon,
    #[token(",", ignore(ascii_case))]
    Comma,

    #[token(">", ignore(ascii_case))]
    Greater,
    #[token(">=", ignore(ascii_case))]
    GreaterEqual,
    #[token("<", ignore(ascii_case))]
    Less,
    #[token("<=", ignore(ascii_case))]
    LessEqual,
    #[token("=", ignore(ascii_case))]
    Equal,
    #[token("<>", ignore(ascii_case))]
    NotEqual,

    #[token("and", ignore(ascii_case))]
    AndOp,
    #[token("or", ignore(ascii_case))]
    OrOp,
    #[token("not", ignore(ascii_case))]
    NotOp,

    #[token("+", ignore(ascii_case))]
    Plus,
    #[token("-", ignore(ascii_case))]
    Minus,
    #[token("*", ignore(ascii_case))]
    Multiply,
    #[token("/", ignore(ascii_case))]
    #[token("div", ignore(ascii_case))]
    Divide,
    #[token("mod", ignore(ascii_case))]
    Reminder,

    #[token("if", ignore(ascii_case))]
    IfStatement,
    #[token("then", ignore(ascii_case))]
    IfStatementCodeBlock,
    #[token("else", ignore(ascii_case))]
    ElseStatement,

    #[token("for", ignore(ascii_case))]
    ForLoop,
    #[token("to", ignore(ascii_case))]
    ForLoopIncrease,
    #[token("downto", ignore(ascii_case))]
    ForLoopDecrease,
    #[token("do", ignore(ascii_case))]
    LoopStatementCodeBlock,
    #[token("while", ignore(ascii_case))]
    WhileLoop,
    #[token("repeat", ignore(ascii_case))]
    RepeatLoop,
    #[token("until", ignore(ascii_case))]
    RepeatLoopUntil,

    #[token("nil", ignore(ascii_case))]
    NullValue,

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

//[program .*;].*/i
