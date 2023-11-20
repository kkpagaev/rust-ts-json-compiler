use rust_ts_json_compiler::{
    lexer::{Lexer, Token},
    syntax_tree::SyntaxTree,
    to_json,
};

pub fn main() -> anyhow::Result<()> {
    // let schema = "z.coerce.number()";
    let schema = "z.object({
  id: z.coerce.number().int(),
  isActive: z.coerce.boolean(),
  email: z.coerce.string().email(),
    anyhow: z.any(),
  uuid: z.coerce.string().uuid(),
  ids: z.array(z.coerce.number().int()),
    nested: z.object({
  id: z.coerce.number().int(),
  email: z.coerce.string().email(),
  uuid: z.coerce.string().uuid(),
  ids: z.array(z.coerce.number().int()),
  orderBy: z
    .enum([\"id\", \"status\", \"createdAt\", \"updatedAt\", \"totalPrice\"])
    .optional(),
products: z.array(
          z.object({
            productId: z.number().int(),
            amount: z.number().int(),
            price: z.number()
          })
        ),
    lit: z.literal(\"CREATED\"),
  attributeValues: z
    .union([z.array(z.coerce.number().int()), z.coerce.number().int()])
})
})";
    let mut lx = Lexer::new(schema);
    let mut tokens = Vec::new();

    loop {
        let token = lx.next_token();
        if token == Token::Eof {
            break;
        }

        tokens.push(token);
    }

    // println!("{:?}", tokens);

    let mut tree = SyntaxTree::new(tokens.into_iter().peekable());

    println!("{}", to_json(&tree.parse().unwrap()));

    Ok(())
}
