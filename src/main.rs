mod token;
mod scanner;
mod parser;
mod interpreter;
mod database;
mod relation;
mod graph;

use scanner::{scanner};
use parser::{Parser};
use interpreter::{Interpreter};

use std::fs;
use std::io;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No file provided!");
        std::process::exit(1);
    }

    let file_path = &args[1];

    if !file_path.ends_with(".dl") {
        eprintln!("Error: File '{}' does not have a '.dl' extension.", file_path);
        std::process::exit(1);
    }

    let file_contents = match read_file_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            eprintln!("File probably doesn't exist!");
            std::process::exit(1);
        }
    };

    let tokens = scanner(file_contents);
    let mut parser = Parser::new(tokens);
    let program_data = parser.parse();
    let mut interpreter = Interpreter::new(program_data);
    interpreter.run_interpreter();
}

fn read_file_to_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}
