use std::fmt;

use crate::zk_examples::ZkExample;
use aiken_lang::parser::error::Pattern;
use aiken_lang::parser::token::Base;
use aiken_lang::parser::token::Token;

#[derive(Clone, Debug, PartialEq, PartialOrd, Ord, Hash, Eq)]
pub enum TokenZK {
    Offchain { example: ZkExample },
    Error(char),
    Name { name: String },
    Ordinal { index: u32 },
    UpName { name: String },
    DiscardName { name: String },
    Int { value: String, base: Base },
    ByteString { value: String },
    String { value: String },
    // Groupings
    NewLineLeftParen, // ↳(
    LeftParen,        // (
    RightParen,       // )
    LeftSquare,       // [
    RightSquare,      // }
    LeftBrace,        // {
    RightBrace,       // }
    // Int Operators
    Plus,
    Minus,
    NewLineMinus,
    Star,
    Slash,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Percent,
    // ByteString Operators
    PlusDot,         // '+.'
    MinusDot,        // '-.'
    StarDot,         // '*.'
    SlashDot,        // '/.'
    LessDot,         // '<.'
    GreaterDot,      // '>.'
    LessEqualDot,    // '<=.'
    GreaterEqualDot, // '>=.'
    // Other Punctuation
    Colon,
    Comma,
    Hash,     // '#'
    Bang,     // '!'
    Question, // '?'
    Equal,
    EqualEqual,  // '=='
    NotEqual,    // '!='
    Vbar,        // '|'
    VbarVbar,    // '||'
    AmperAmper,  // '&&'
    And,         // and
    Or,          // or
    NewLinePipe, // '↳|>'
    Pipe,        // '|>'
    Dot,         // '.'
    RArrow,      // '->'
    LArrow,      // '<-'
    DotDot,      // '..'
    EndOfFile,
    // Docs/Extra
    Comment,
    DocComment,
    ModuleComment,
    EmptyLine,
    NewLine,
    // Keywords (alphabetically):
    As,
    Benchmark,
    Const,
    Fn,
    If,
    Else,
    Fail,
    Once,
    Expect,
    Is,
    Let,
    Opaque,
    Pub,
    Use,
    Test,
    Todo,
    Type,
    When,
    Trace,
    Validator,
    Via,
}

impl fmt::Display for TokenZK {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let index_str;
        let s = match self {
            TokenZK::Offchain { example } => &format!("[offchain] -> {:?}", example),
            TokenZK::Error(c) => {
                write!(f, "\"{c}\"")?;
                return Ok(());
            }
            TokenZK::Name { name } => name,
            TokenZK::Ordinal { index } => {
                index_str = index.to_string();
                &index_str[..]
            }
            TokenZK::UpName { name } => name,
            TokenZK::DiscardName { name } => name,
            TokenZK::Int { value, .. } => value,
            TokenZK::String { value } => value,
            TokenZK::ByteString { value } => value,
            TokenZK::NewLineLeftParen => "↳(",
            TokenZK::LeftParen => "(",
            TokenZK::RightParen => ")",
            TokenZK::LeftSquare => "[",
            TokenZK::RightSquare => "]",
            TokenZK::LeftBrace => "{",
            TokenZK::RightBrace => "}",
            TokenZK::Plus => "+",
            TokenZK::Minus => "-",
            TokenZK::NewLineMinus => "↳-",
            TokenZK::Star => "*",
            TokenZK::Slash => "/",
            TokenZK::Less => "<",
            TokenZK::Greater => ">",
            TokenZK::LessEqual => "<=",
            TokenZK::GreaterEqual => ">=",
            TokenZK::Percent => "%",
            TokenZK::PlusDot => "+.",
            TokenZK::MinusDot => "-.",
            TokenZK::StarDot => "*.",
            TokenZK::SlashDot => "/.",
            TokenZK::LessDot => "<.",
            TokenZK::GreaterDot => ">.",
            TokenZK::LessEqualDot => "<=.",
            TokenZK::GreaterEqualDot => ">=.",
            TokenZK::Colon => ":",
            TokenZK::Comma => ",",
            TokenZK::Hash => "#",
            TokenZK::Bang => "!",
            TokenZK::Equal => "=",
            TokenZK::Question => "?",
            TokenZK::EqualEqual => "==",
            TokenZK::NotEqual => "!=",
            TokenZK::Vbar => "|",
            TokenZK::VbarVbar => "||",
            TokenZK::AmperAmper => "&&",
            TokenZK::And => "and",
            TokenZK::Or => "or",
            TokenZK::NewLinePipe => "↳|>",
            TokenZK::Pipe => "|>",
            TokenZK::Dot => ".",
            TokenZK::RArrow => "->",
            TokenZK::LArrow => "<-",
            TokenZK::DotDot => "..",
            TokenZK::EndOfFile => "EOF",
            TokenZK::Comment => "//",
            TokenZK::DocComment => "///",
            TokenZK::ModuleComment => "////",
            TokenZK::EmptyLine => "EMPTYLINE",
            TokenZK::NewLine => "NEWLINE",
            TokenZK::As => "as",
            TokenZK::Expect => "expect",
            TokenZK::When => "when",
            TokenZK::Is => "is",
            TokenZK::Const => "const",
            TokenZK::Fn => "fn",
            TokenZK::If => "if",
            TokenZK::Else => "else",
            TokenZK::Use => "use",
            TokenZK::Let => "let",
            TokenZK::Opaque => "opaque",
            TokenZK::Pub => "pub",
            TokenZK::Todo => "todo",
            TokenZK::Trace => "trace",
            TokenZK::Type => "type",
            TokenZK::Test => "test",
            TokenZK::Fail => "fail",
            TokenZK::Once => "once",
            TokenZK::Validator => "validator",
            TokenZK::Via => "via",
            TokenZK::Benchmark => "bench",
        };
        write!(f, "{s}")
    }
}

impl From<TokenZK> for Pattern {
    fn from(_: TokenZK) -> Self {
        Self::Token(Token::Plus)
    }
}
