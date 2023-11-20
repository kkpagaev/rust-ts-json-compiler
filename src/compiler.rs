use crate::{
    lexer::{Lexer, Token},
    syntax_tree::{SyntaxTree, ZodExpression},
};

pub fn get_syntax_tree(schema: &str) -> Option<ZodExpression> {
    let mut lx = Lexer::new(schema);
    let mut tokens = Vec::new();

    loop {
        let token = lx.next_token();
        if token == Token::Eof {
            break;
        }

        tokens.push(token);
    }

    SyntaxTree::new(tokens.into_iter().peekable()).parse()
}

pub fn to_json(zod: &ZodExpression) -> String {
    match zod {
        ZodExpression::Object(obj) => {
            let mut json = String::new();
            json.push('{');
            json.push_str(
                &obj.iter()
                    .map(|(key, value)| format!("\"{}\": {}", key, to_json(value)))
                    .collect::<Vec<String>>()
                    .join(", "),
            );

            json.push('}');
            json
        }
        ZodExpression::Number =>  "1".to_string(),
        ZodExpression::String =>  "\"string\"".to_string(),
        ZodExpression::UUID =>  "\"aa5ac446-7e1d-11ee-b962-0242ac120002\"".to_string(),
        ZodExpression::Boolean =>  "true".to_string(),
        ZodExpression::Array(array) => {
            let mut json = String::new();
            json.push('[');
            json.push_str(&to_json(array));
            json.push(']');
            json
        }
        ZodExpression::Literal(l) =>  format!("\"{}\"", l),
        ZodExpression::Email =>  "\"admin@admin.com\"".to_string(),
        ZodExpression::Any =>  "{}".to_string(),
        ZodExpression::Enum(e) =>  format!("\"{}\"", e.first().unwrap()),
        ZodExpression::Union(u) =>  to_json(u.first().unwrap()),
    }
}
