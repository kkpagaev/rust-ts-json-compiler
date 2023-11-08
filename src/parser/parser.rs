use crate::lexer::Token;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Parser {
    tokens: Vec<Token>,
}
