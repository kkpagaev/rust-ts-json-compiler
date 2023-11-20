use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(String),
    Comma,
    LRound,
    RRound,
    LCurly,
    RCurly,
    LSquare,
    RSquare,
    True,
    False,
    Str(String),
    Dot,
    Colon,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Illegal => write!(f, ""),
            Token::Eof => write!(f, "\0"),
            Token::Ident(value) | Token::Int(value) | Token::Str(value) => write!(f, "{value}"),
            Token::Comma => write!(f, ","),
            Token::LRound => write!(f, "("),
            Token::RRound => write!(f, ")"),
            Token::LCurly => write!(f, "{{"),
            Token::RCurly => write!(f, "}}"),
            Token::LSquare => write!(f, "["),
            Token::RSquare => write!(f, "]"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Dot => write!(f, "."),
            Token::Colon => write!(f, ":"),
        }
    }
}

impl From<char> for Token {
    fn from(ch: char) -> Self {
        match ch {
            '(' => Self::LRound,
            ')' => Self::RRound,
            ',' => Self::Comma,
            '{' => Self::LCurly,
            '}' => Self::RCurly,
            '[' => Self::LSquare,
            ']' => Self::RSquare,
            '.' => Self::Dot,
            ':' => Self::Colon,
            '\0' => Self::Eof,
            _ => Self::Illegal,
        }
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        match value.as_str() {
            "true" => Self::True,
            "false" => Self::False,
            _ => {
                if value.chars().all(|b| b.is_ascii_digit() || b == '.') {
                    Self::Int(value)
                } else if (value.starts_with('"') && value.ends_with('"'))
                    || (value.starts_with('\'') && value.ends_with('\''))
                {
                    Self::Str(value)
                } else {
                    Self::Ident(value)
                }
            }
        }
    }
}
