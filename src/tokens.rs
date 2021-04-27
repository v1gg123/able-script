use logos::{Lexer, Logos};

use crate::variables::Abool;

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token {
    // Literals
    /// True, False
    #[regex("true|false", get_bool)]
    Boolean(bool),

    /// Always, Sometimes, Never
    #[regex("always|sometimes|never", get_abool)]
    Aboolean(Abool),

    /// String
    #[regex("\"(\\.|[^\"])*\"", get_string)]
    String(String),

    /// Integer
    #[regex(r"[0-9]+", get_int)]
    Integer(i32),

    /// A C-complaint identifier
    #[regex(r"[a-zA-Z_][a-zA-Z_0-9]*")]
    Identifier,

    #[token("(")]
    LeftParenthesis,

    #[token(")")]
    RightParenthesis,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token(";")]
    Semicolon,

    #[token(".")]
    FullStop,

    #[token(",")]
    Comma,

    #[regex(r"#.*")]
    Comment,

    // Operators
    #[token("-")]
    Subtract,

    #[token("+")]
    Addition,

    #[token("*")]
    Multiply,

    #[token("/")]
    Divide,

    #[token("=")]
    Assignment,

    // Logical operators
    #[token("<")]
    OpLt,

    #[token(">")]
    OpGt,

    #[token("==")]
    OpEq,

    #[token("!=")]
    OpNeq,

    /// Base52 based character ('a')
    #[token("'.*'")]
    Char,

    #[token("functio")]
    Function,

    /// Brain fuck FFI
    #[token("bff")]
    BfFunction,

    /// Variable bro
    #[token("var")]
    Variable,

    /// Prints the preceding things
    #[token("print")]
    Print,

    /// Ban the following variable from ever being used again
    #[token("melo")]
    Melo,

    #[token("T-Dark")]
    TDark,

    // Expressions
    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token("loop")]
    Loop,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,
}

fn get_bool(lexer: &mut Lexer<Token>) -> Option<bool> {
    lexer.slice().parse().ok()
}

fn get_int(lexer: &mut Lexer<Token>) -> Option<i32> {
    lexer.slice().parse().ok()
}

fn get_string(lexer: &mut Lexer<Token>) -> String {
    lexer.slice().to_owned()
}

fn get_abool(lexer: &mut Lexer<Token>) -> Option<Abool> {
    match lexer.slice() {
        "always" => Some(Abool::Always),
        "sometimes" => Some(Abool::Sometimes),
        "never" => Some(Abool::Never),
        _ => None,
    }
}
