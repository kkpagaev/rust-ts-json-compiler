use std::iter::Peekable;
use std::str::Chars;

use super::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,

    ch: u8,
}

#[allow(dead_code)]
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input: input.chars().peekable(),
            ch: 0,
        };
        lexer.next_char();
        lexer
    }

    fn next_char(&mut self) {
        self.ch = match self.input.peek() {
            Some(ch) => *ch as u8,
            None => 0,
        };

        self.input.next();
    }

    pub fn peek(&mut self) -> u8 {
        match self.input.peek() {
            Some(ch) => *ch as u8,
            None => 0,
        }
    }

    pub fn next(&mut self) -> Token {
        self.skip_whitespace();

        let ch = match self.ch {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.consume_ident(),
            b'.' => {
                self.next_char();
                Token::Dot
            }
            b':' => {
                self.next_char();
                Token::Colon
            }
            b'0'..=b'9' => self.consume_int(),
            b',' => {
                self.next_char();
                Token::Comma
            }
            b'(' => {
                self.next_char();
                Token::LRound
            }
            b')' => {
                self.next_char();
                Token::RRound
            }
            b'{' => {
                self.next_char();
                Token::LCurly
            }
            b'}' => {
                self.next_char();
                Token::RCurly
            }
            b'[' => {
                self.next_char();
                Token::LSquare
            }
            b']' => {
                self.next_char();
                Token::RSquare
            }
            b'"' | b'\'' => self.consume_string(),
            0 => Token::Eof,
            _ => {
                self.next_char();
                Token::Illegal
            }
        };

        println!("Token: {:?}", ch);
        ch
    }

    fn consume_string(&mut self) -> Token {
        let mut value = String::new();
        let quote_type = self.ch;
        let mut next_skip = false;

        self.next_char();

        while self.ch != quote_type || next_skip {
            next_skip = self.ch == b'\\';
            value.push(self.ch as char);
            self.next_char();
        }

        self.next_char();

        Token::Ident(value)
    }

    fn consume_ident(&mut self) -> Token {
        let mut value = String::new();

        while self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
            value.push(self.ch as char);
            self.next_char();
        }

        Token::Ident(value)
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.ch {
                b' ' | b'\t' | b'\n' | b'\r' => {
                    self.next_char();
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn consume_int(&mut self) -> Token {
        let mut value = String::new();

        while self.ch.is_ascii_digit() || self.ch == b'.' {
            value.push(self.ch as char);
            self.next_char();
        }

        Token::Int(value)
    }
}
