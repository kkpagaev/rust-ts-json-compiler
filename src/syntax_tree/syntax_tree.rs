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
                        Err(e) => {
                            println!("{:?}", e);
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
            "literal" => Ok(ZodExpression::Literal("".to_string())),
            "number" => self.parse_zod_number(),
            "string" => self.parse_zod_string(),
            "boolean" => self.parse_zod_boolean(),
            "coerce" => {
                self.parse_zod()
            },
            _ => Err(anyhow!("Unexpected token")),
        }
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
            match self.next() {
                Some(Token::RCurly) => {
                    break;
                }
                Some(Token::Comma) => {
                    continue;
                }
                Some(Token::Ident(ref ident)) => {
                    println!("ident {:?}", ident);
                    self.parse_colon()?;
                    let exp = match self.parse() {
                        Some(e) => e,
                        None => break
                    };
                    println!("exp {:?}", exp);
                    obj.push((ident.to_owned(), exp));
                }
                _ => return Err(anyhow!("Unexpected token in parse_zod_object_body")),
            }

            println!("{}", obj.len());
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

    fn parse_dot(&mut self) -> Result<()> {
        match self.next() {
            Some(Token::Dot) => Ok(()),
            _ => Err(anyhow!("Unexpected token parse_dot")),
        }
    }

    fn next(&mut self) -> Option<Token> {
        println!("{}", self.tokens.peek().unwrap());
        self.tokens.next()
    }
}
