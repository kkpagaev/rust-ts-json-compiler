use rust_ts_json_compiler::{lexer::{Lexer, Token}, syntax_tree::SyntaxTree};

pub fn main() -> anyhow::Result<()> {
    // let schema = "z.coerce.number()";
    let schema = "    z.object({
        products: z.array(
          z.object({
            productId: z.number().int(),
            amount: z.number().int(),
            price: z.number()
          })
        ),
        cityId: z.number().int(),
        comment: z.string()
    })";
    let mut lx = Lexer::new(schema);
    let mut tokens = Vec::new();

    loop {
        let token = lx.next();
        if token == Token::Eof {
            break;
        }

        tokens.push(token);
    }

    // println!("{:?}", tokens);

    let mut tree = SyntaxTree::new(tokens.into_iter().peekable());

    println!("foo {:?}", tree.parse().unwrap());


    return Ok(());
}
