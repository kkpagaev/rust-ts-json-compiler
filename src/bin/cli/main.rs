use rust_ts_json_compiler::{get_syntax_tree, to_json};
use std::env;

pub fn main() -> anyhow::Result<()> {
    // get arg from cli
    let args = env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        println!("Usage: rust-ts-json-compiler <path_to_schema>");
        return Ok(());
    }

    let file_content = std::fs::read_to_string(&args[1])?;

    let json = to_json(&get_syntax_tree(&file_content).unwrap());

    println!("{}", json);

    return Ok(());
}
