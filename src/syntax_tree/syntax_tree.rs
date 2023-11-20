use std::iter::Peekable;
use std::vec;
use thiserror::Error;

use crate::lexer::Token;

#[derive(Debug)]
#[allow(dead_code)]
pub enum ZodExpression {
    Object(Box<Vec<(String, ZodExpression)>>),
    Array(Box<ZodExpression>),
    Literal(String),
    Number,
    UUID,
    String,
    Boolean,
    Email,
    Any,
    Enum(Vec<String>),
    Union(Vec<ZodExpression>),
}

#[derive(Error, Debug)]
pub enum SyntaxError {
    #[error("Expected token {0:?} found {1:?}")]
    UnexpectedToken(Token, Token),

    #[error("Invalid identifier {0:?}")]
    InvalidIdentifier(String),

    #[error("Unexpected end of a file")]
    UnexpectedEndOfFile,

    #[error("Unexpected token in enum {0:?}")]
    UnexpectedTokenInEnum(Token),

    #[error("Unexpected token in object body {0:?}")]
    UnexpectedTokenInObjectBody(Token),
}

pub struct SyntaxTree {
    tokens: Peekable<vec::IntoIter<Token>>,
}

impl SyntaxTree {
    pub fn new(tokens: Peekable<vec::IntoIter<Token>>) -> SyntaxTree {
        SyntaxTree { tokens }
    }

    pub fn parse(&mut self) -> Option<ZodExpression> {
        match self.tokens.peek() {
            Some(Token::Ident(ident)) => {
                if ident == "z" {
                    match self.parse_zod() {
                        Result::Ok(zod) => Some(zod),
                        Err(_) => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn parse_zod(&mut self) -> Result<ZodExpression, SyntaxError> {
        self.next();
        self.parse_dot()?;
        let ident = match self.tokens.peek() {
            Some(Token::Ident(ident)) => ident,
            Some(_) => {
                return Err(SyntaxError::UnexpectedToken(
                    self.tokens.next().unwrap(),
                    Token::Ident("".to_string()),
                ))
            }
            None => return Err(SyntaxError::UnexpectedEndOfFile),
        };

        match ident.as_str() {
            "object" => self.parse_zod_object_body(),
            "array" => self.parse_zod_array(),
            "literal" => self.parse_zod_literal(),
            "number" => self.parse_zod_number(),
            "enum" => self.parse_zod_enum(),
            "string" => self.parse_zod_string(),
            "boolean" => self.parse_zod_boolean(),
            "any" => self.parse_zod_any(),
            "union" => self.parse_zod_union(),
            "coerce" => self.parse_zod(),
            _ => Err(SyntaxError::InvalidIdentifier(ident.to_string())),
        }
    }

    fn parse_zod_literal(&mut self) -> Result<ZodExpression, SyntaxError> {
        self.next();
        self.parse_left_round()?;
        match self.next() {
            Some(Token::Str(value)) => {
                self.parse_right_round()?;
                Ok(ZodExpression::Literal(value))
            }
            Some(token) => Err(SyntaxError::UnexpectedToken(
                token,
                Token::Str("\"\"".to_string()),
            )),
            None => Err(SyntaxError::UnexpectedEndOfFile),
        }
    }

    fn parse_zod_union(&mut self) -> Result<ZodExpression, SyntaxError> {
        self.next();
        self.parse_left_round()?;
        self.parse_left_square()?;

        let mut arr = vec![];

        loop {
            match self.parse() {
                Some(e) => arr.push(e),
                None => break,
            };

            match self.next() {
                Some(Token::RSquare) => break,
                Some(Token::Comma) => continue,
                Some(token) => return Err(SyntaxError::UnexpectedToken(token, Token::RSquare)),
                None => return Err(SyntaxError::UnexpectedEndOfFile),
            };
        }
        self.parse_right_round()?;
        self.parse_to_end_of_scope();

        Ok(ZodExpression::Union(arr))
    }

    fn parse_zod_enum(&mut self) -> Result<ZodExpression, SyntaxError> {
        self.next();
        self.parse_left_round()?;
        self.parse_left_square()?;

        let mut arr = vec![];

        loop {
            let token = self.next();
            match token {
                Some(Token::RSquare) => {
                    break;
                }
                Some(Token::Comma) => {
                    continue;
                }
                Some(Token::Str(ref str)) => {
                    arr.push(str.to_owned());
                }
                None => {
                    return Err(SyntaxError::UnexpectedEndOfFile);
                }
                Some(token) => {
                    return Err(SyntaxError::UnexpectedTokenInEnum(token));
                }
            }
        }
        self.parse_right_round()?;
        self.parse_to_end_of_scope();

        Ok(ZodExpression::Enum(arr))
    }

    fn parse_zod_any(&mut self) -> Result<ZodExpression, SyntaxError> {
        self.next();
        self.parse_left_round()?;
        self.parse_right_round()?;

        Ok(ZodExpression::Any)
    }

    fn parse_zod_boolean(&mut self) -> Result<ZodExpression, SyntaxError> {
        self.next();
        self.parse_left_round()?;
        self.parse_right_round()?;

        Ok(ZodExpression::Boolean)
    }

    fn parse_zod_array(&mut self) -> Result<ZodExpression, SyntaxError> {
        self.next();
        self.parse_left_round()?;
        let exp = match self.parse() {
            Some(e) => e,
            None => return Err(SyntaxError::UnexpectedEndOfFile),
        };
        self.parse_to_end_of_scope();

        Ok(ZodExpression::Array(Box::new(exp)))
    }
    fn parse_zod_number(&mut self) -> Result<ZodExpression, SyntaxError> {
        self.next();
        self.parse_left_round()?;
        self.parse_right_round()?;
        self.parse_to_end_of_scope();
        Ok(ZodExpression::Number)
    }

    fn parse_zod_string(&mut self) -> Result<ZodExpression, SyntaxError> {
        self.next();
        self.parse_left_round()?;
        self.parse_right_round()?;

        loop {
            if self.tokens.peek() != Some(&Token::Dot) {
                break;
            }

            self.next();
            let iden = match self.tokens.peek() {
                Some(Token::Ident(ident)) => ident,
                Some(_) => {
                    return Err(SyntaxError::UnexpectedToken(
                        self.next().unwrap(),
                        Token::Ident("".to_string()),
                    ))
                }
                None => return Err(SyntaxError::UnexpectedEndOfFile),
            };
            if iden == "email" {
                self.parse_to_end_of_scope();
                return Ok(ZodExpression::Email);
            }
            if iden == "uuid" {
                self.parse_to_end_of_scope();
                return Ok(ZodExpression::UUID);
            }
        }
        self.parse_to_end_of_scope();
        Ok(ZodExpression::String)
    }

    fn parse_to_end_of_scope(&mut self) {
        loop {
            match self.tokens.peek() {
                Some(token) => match token {
                    Token::Eof | Token::RSquare | Token::Comma | Token::RCurly => {
                        break;
                    }
                    _ => {
                        self.next();
                        continue;
                    }
                },
                None => {
                    break;
                }
            }
        }
    }

    fn parse_zod_object_body(&mut self) -> Result<ZodExpression, SyntaxError> {
        self.next();
        self.parse_left_round()?;
        self.parse_left_curly()?;
        let mut obj = vec![];

        loop {
            let token = self.next();
            match token {
                Some(Token::RCurly) => {
                    break;
                }
                Some(Token::RRound) => {
                    continue;
                }
                Some(Token::Comma) => {
                    continue;
                }
                Some(Token::Ident(ref ident)) => {
                    self.parse_colon()?;
                    let exp = match self.parse() {
                        Some(e) => e,
                        None => break,
                    };
                    obj.push((ident.to_owned(), exp));
                }
                Some(token) => {
                    return Err(SyntaxError::UnexpectedTokenInObjectBody(token));
                }
                None => {
                    return Err(SyntaxError::UnexpectedEndOfFile);
                }
            }
        }
        self.parse_right_round()?;

        Ok(ZodExpression::Object(Box::new(obj)))
    }

    fn parse_left_round(&mut self) -> Result<(), SyntaxError> {
        match self.next() {
            Some(Token::LRound) => Ok(()),
            Some(token) => Err(SyntaxError::UnexpectedToken(token, Token::LRound)),
            None => Err(SyntaxError::UnexpectedEndOfFile),
        }
    }

    fn parse_right_round(&mut self) -> Result<(), SyntaxError> {
        match self.next() {
            Some(Token::RRound) => Ok(()),
            Some(token) => Err(SyntaxError::UnexpectedToken(token, Token::RRound)),
            None => Err(SyntaxError::UnexpectedEndOfFile),
        }
    }

    fn parse_left_curly(&mut self) -> Result<(), SyntaxError> {
        match self.next() {
            Some(Token::LCurly) => Ok(()),
            Some(token) => Err(SyntaxError::UnexpectedToken(token, Token::LCurly)),
            None => Err(SyntaxError::UnexpectedEndOfFile),
        }
    }

    fn parse_colon(&mut self) -> Result<(), SyntaxError> {
        match self.next() {
            Some(Token::Colon) => Ok(()),
            Some(token) => Err(SyntaxError::UnexpectedToken(token, Token::Colon)),
            None => Err(SyntaxError::UnexpectedEndOfFile),
        }
    }

    fn parse_left_square(&mut self) -> Result<(), SyntaxError> {
        match self.next() {
            Some(Token::LSquare) => Ok(()),
            Some(token) => Err(SyntaxError::UnexpectedToken(token, Token::LSquare)),
            None => Err(SyntaxError::UnexpectedEndOfFile),
        }
    }

    fn parse_dot(&mut self) -> Result<(), SyntaxError> {
        match self.next() {
            Some(Token::Dot) => Ok(()),
            Some(token) => Err(SyntaxError::UnexpectedToken(token, Token::Dot)),
            None => Err(SyntaxError::UnexpectedEndOfFile),
        }
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }
}
