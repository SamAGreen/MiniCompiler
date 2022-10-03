pub mod tokenizer;
pub mod ast;
mod parser;

fn main() {
    println!("Hello, world!");
    ast::test_pretty()
    //tokenizer::test_tokenize();
}
