use logos::{Lexer, Logos};

fn san_to_text(san_lex: &mut Lexer<SanTokenKinds>) -> Option<String> {
    Some(san_lex.slice().trim().to_string())
}

fn san_to_number(san_lex: &mut Lexer<SanTokenKinds>) -> Option<f64> {
    san_lex.slice().parse::<f64>().ok()
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum SanTokenKinds {
    SanPascalNewLine,

    #[regex(r###"program.*"###, ignore(ascii_case))]
    PascalProgramStart,
    #[token("end.", ignore(ascii_case))]
    PascalProgramEnd,
    #[token("begin", ignore(ascii_case))]
    PascalCodeBlockBegin,
    #[token("end", ignore(ascii_case))]
    PascalCodeBlockEnd,
    #[token("procedure", ignore(ascii_case))]
    #[token("function", ignore(ascii_case))]
    ProcedureFunc,
    #[regex(r###"[\{\.*}].*"###, san_to_text)]
    Comment(String),

    #[token("var", ignore(ascii_case))]
    LetDeclare,
    #[token("const", ignore(ascii_case))]
    ConstDeclare,
    #[token(":=", ignore(ascii_case))]
    AssignVar,

    //TODO
    #[token("char", ignore(ascii_case))]
    #[token("string", ignore(ascii_case))]
    DataTypeString,
    #[token("shortint", ignore(ascii_case))]
    #[token("smallint", ignore(ascii_case))]
    #[token("integer", ignore(ascii_case))]
    #[token("longint", ignore(ascii_case))]
    #[token("real", ignore(ascii_case))]
    #[token("number", ignore(ascii_case))]
    DataTypeNumber,
    #[token("boolean", ignore(ascii_case))]
    DataTypeBoolean,
    #[regex(r###"|array.*"###)]
    DataTypeArray,

    #[token("(", ignore(ascii_case))]
    LeftParen,
    #[token(")", ignore(ascii_case))]
    RightParen,
    #[token("[", ignore(ascii_case))]
    LeftSqrParen,
    #[token("]", ignore(ascii_case))]
    RightSqrParen,
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
    #[token("~", ignore(ascii_case))]
    BitNo,

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

    #[token("write", ignore(ascii_case))]
    #[token("writeln", ignore(ascii_case))]
    OutputWriteFunc,
    #[token("read", ignore(ascii_case))]
    #[token("readln", ignore(ascii_case))]
    InputReadFunc,
    #[token("html", ignore(ascii_case))]
    #[token("htmlln", ignore(ascii_case))]
    #[token("htmln", ignore(ascii_case))]
    HtmlOutputFunc,
    #[token("clear()", ignore(ascii_case))]
    #[token("clearln()", ignore(ascii_case))]
    #[token("clean()", ignore(ascii_case))]
    #[token("cleanln()", ignore(ascii_case))]
    HtmlOutputClearFunc,

    #[regex(r"[a-zA-Z_?]+", san_to_text)]
    Identifier(String),

    #[regex(r"([0-9]+[.])?[0-9]+", san_to_number)]
    Number(f64),

    #[regex(r##""(?:[^"\\]|\\.)*""##, san_to_text)]
    Text(String),

    #[token("true", ignore(ascii_case))]
    SanTrue,
    #[token("false", ignore(ascii_case))]
    SanFalse,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    SanError,
}
