use rust_ts_json_compiler::lexer::{Lexer, Token};

pub fn main() {
    let schema = "z.object({
  id: z.coerce.number()
})";
    let mut lexel = Lexer::new(schema);
    let mut tokens = Vec::new();

    loop {
        let token = lexel.next();
        if token == Token::Eof {
            break;
        }

        tokens.push(token);
    }

    println!("{:?}", tokens);
}
