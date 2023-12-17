use rust_ts_json_compiler::{get_syntax_tree, to_json};
use std::io::{self, Read};

pub fn main() -> anyhow::Result<()> {
    // get arg from cli
    // let args = env::args().collect::<Vec<String>>();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    // if args.len() != 2 {
    //     println!("Usage: rust-ts-json-compiler <path_to_schema>");
    //     return Ok(());
    // }

    // let file_content = std::fs::read_to_string(&args[1])?;

    let json = to_json(&get_syntax_tree(&buffer).unwrap());

    println!("{}", json);

    Ok(())
}
