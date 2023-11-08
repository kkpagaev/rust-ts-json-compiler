use std::iter::Peekable;
use std::vec;

use crate::lexer::Token;
use anyhow::{anyhow, Ok, Result};

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
                        Result::Ok(zod) => {
                            Some(zod)
                        },
                        Err(_) => {
                            None
                        }
                    }
                } else {
                    None
                }
            }
            _ => None
        }
    }

    fn parse_zod(&mut self) -> Result<ZodExpression> {
        self.next();
        self.parse_dot()?;
        let ident = match self.tokens.peek() {
            Some(Token::Ident(ident)) => ident,
            _ => return Err(anyhow!("Unexpected token in parse_zod")),
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
            "coerce" => {
                self.parse_zod()
            },
            _ => Err(anyhow!("Unexpected token")),
        }
    }

    fn parse_zod_literal(&mut self) -> Result<ZodExpression> {
        self.next();
        self.parse_left_round()?;
        match self.next() {
            Some(Token::Str(value)) => {
                self.parse_right_round()?;
                Ok(ZodExpression::Literal(value))
            }
            _ => Err(anyhow!("Unexpected token in parse_zod_literal")),
        }
    }

    fn parse_zod_union(&mut self) -> Result<ZodExpression> {
        self.next();
        self.parse_left_round()?;
        self.parse_left_square()?;

        let mut arr = vec![];

        loop {
            match self.parse() {
                Some(e) => arr.push(e),
                None => break
            };

            match self.next() {
                Some(Token::RSquare) => break,
                Some(Token::Comma) => continue,
                _ => return Err(anyhow!("Unexpected token in parse_zod_enum"))
            };
        }
        self.parse_right_round()?;
        self.parse_to_end_of_scope()?;

        Ok(ZodExpression::Union(arr))
    }

    fn parse_zod_enum(&mut self) -> Result<ZodExpression> {
        self.next();
        self.parse_left_round()?;
        self.parse_left_square()?;

        let mut arr = vec![];

        loop {
            let token = self.next();
            match token  {
                Some(Token::RSquare) => {
                    break;
                }
                Some(Token::Comma) => {
                    continue;
                }
                Some(Token::Str(ref str)) => {
                    arr.push(str.to_owned());
                }
                _ => return Err(anyhow!("Unexpected token in parse_zod_enum {}", token.unwrap())),
            }
        }
        self.parse_right_round()?;
        self.parse_to_end_of_scope()?;

        Ok(ZodExpression::Enum(arr))
    }

    fn parse_zod_any(&mut self) -> Result<ZodExpression> {
        self.next();
        self.parse_left_round()?;
        self.parse_right_round()?;

        Ok(ZodExpression::Any)
    }

    fn parse_zod_boolean(&mut self) -> Result<ZodExpression> {
        self.next();
        self.parse_left_round()?;
        self.parse_right_round()?;

        Ok(ZodExpression::Boolean)
    }

    fn parse_zod_array(&mut self) -> Result<ZodExpression> {
        self.next();
        self.parse_left_round()?;
        let exp = match self.parse() {
            Some(e) => e,
            None => return Err(anyhow!("Unexpected token in parse_zod_array")),
        };

        Ok(ZodExpression::Array(Box::new(exp)))
    }
    fn parse_zod_number(&mut self) -> Result<ZodExpression> {
        self.next();
        self.parse_left_round()?;
        self.parse_right_round()?;
        self.parse_to_end_of_scope()?;
        Ok(ZodExpression::Number)
    }

    fn parse_zod_string(&mut self) -> Result<ZodExpression> {
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
                _ => return Err(anyhow!("Unexpected token in parse_zod_string")),
            };
            if iden == "email" {
                self.parse_to_end_of_scope()?;
                return Ok(ZodExpression::Email);
            }
            if iden == "uuid" {
                self.parse_to_end_of_scope()?;
                return Ok(ZodExpression::UUID);
            }
        }
        self.parse_to_end_of_scope()?;
        Ok(ZodExpression::String)
    }

    fn parse_to_end_of_scope(&mut self) -> Result<()> {
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

        Ok(())
    }

    fn parse_zod_object_body(&mut self) -> Result<ZodExpression> {
        self.next();
        self.parse_left_round()?;
        self.parse_left_curly()?;
        let mut obj = vec![];

        loop {
            let token = self.next();
            match token  {
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
                        None => break
                    };
                    obj.push((ident.to_owned(), exp));
                }
                _ => return Err(anyhow!("Unexpected token in parse_zod_object_body {}", token.unwrap())),
            }
        }
        self.parse_right_round()?;

        Ok(ZodExpression::Object(Box::new(obj)))
    }

    fn parse_left_round(&mut self) -> Result<()> {
        match self.next() {
            Some(Token::LRound) => Ok(()),
            _ => Err(anyhow!("Unexpected token parse_left_round")),
        }
    }

    fn parse_right_round(&mut self) -> Result<()> {
        match self.next() {
            Some(Token::RRound) => Ok(()),
            _ => Err(anyhow!("Unexpected token parse_right_round")),
        }
    }

    fn parse_left_curly(&mut self) -> Result<()> {
        match self.next() {
            Some(Token::LCurly) => Ok(()),
            _ => Err(anyhow!("Unexpected token parse_left_curly")),
        }
    }

    // fn parse_right_curly(&mut self) -> Result<()> {
    //     match self.next() {
    //         Some(Token::RCurly) => Ok(()),
    //         _ => Err(anyhow!("Unexpected token parse_right_curly")),
    //     }
    // }

    fn parse_colon(&mut self) -> Result<()> {
        match self.next() {
            Some(Token::Colon) => Ok(()),
            _ => Err(anyhow!("Unexpected token parse_colon")),
        }
    }

    fn parse_left_square(&mut self) -> Result<()> {
        match self.next() {
            Some(Token::LSquare) => Ok(()),
            _ => Err(anyhow!("Unexpected token parse_left_square")),
        }
    }

    fn parse_dot(&mut self) -> Result<()> {
        match self.next() {
            Some(Token::Dot) => Ok(()),
            _ => Err(anyhow!("Unexpected token parse_dot")),
        }
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }
}
