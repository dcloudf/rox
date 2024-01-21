use std::fs::read_to_string;
use std::io;
use std::process::exit;

pub mod token;
pub mod scanner;

struct Rox {
    had_error: bool
}

fn error(line: i32, message: String) {
    report(line, "", message)
}

fn report(line: i32, where_str: &str, message: String) {
    println!("[line {}] Error {}: {}", line, where_str, message)
}

impl Rox {
    fn run_file(&self, path: String) {
        Rox::run(self, read_to_string(path).unwrap());
        if self.had_error {
            exit(64);
        }
    }

    fn run_prompt(&mut self) {
        let mut buffer = String::new();
        loop {
            println!("> ");
            io::stdin().read_line(&mut buffer);
            if buffer.is_empty() {
                break;
            }
            self.run(buffer);
            self.had_error = false;
        }
    }

    fn run(&self, source: String) {
        let mut scanner = scanner::Scanner::new(source);
        let tokens: Vec<token::Token> = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }
}