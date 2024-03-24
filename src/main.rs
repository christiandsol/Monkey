use std::env;
mod lexer;
mod token_type;
mod scanner;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default = &String::from("");
    let file_path = args.get(1).unwrap_or(default);
    println!("{}", file_path);
    let contents = lexer::read_file(file_path);
    println!("{}", contents);
}
