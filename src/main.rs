mod lexer;
mod ast;
mod eval;
use std::io;
use std::io::Write;

const PROMPT:&str = "🐒 >>> ";

fn print_prompt() {
   print!("{}", PROMPT);
   io::stdout().flush().unwrap();
}

fn start() {
    let evaluator = &eval::Evaluator::new();
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n > 1 {
                    let lexer = lexer::Lexer::new(&input);
                    let mut parser = ast::Parser::new(lexer);
                    let program = parser.parse();
                    if !parser.errors.is_empty() {
                        for error in parser.errors {
                            println!("{}", error);
                        }
                    } else {
                        let result = evaluator.eval_program(program);
                        println!("{}", result.inspect());
                    }
                }
                print_prompt();
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

fn main() {
    print_prompt();
    start();
}
