pub mod tokenizer;
pub mod ast;
mod parser;
mod vm;

fn main() {
    println!("Hello, world!");
    ast::test_pretty()
}
