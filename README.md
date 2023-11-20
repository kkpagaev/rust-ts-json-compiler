# Rust zod compiler

Compiles [zod](https://github.com/colinhacks/zod) schema to valid json payload.

## Usage

```rust

pub fn main() {
    let zod_schema = "
    z.object({
        products: z.array(
          z.object({
            productId: z.number().int(),
            amount: z.number().int(),
            price: z.number()
          })
        ),
        cityId: z.number().int(),
        comment: z.string()
    })
    ";

    let json_schema = rust_ts_json_compiler::to_json(zod_schema);

    /* 
    {
        "products": [
            {
                "productId": 1,
                "amount": 1,
                "price": 5
            }
        ],
        "cityId": 1,
        "comment": "string"
    }
    */
    println!("{}", json_schema);
}

```


