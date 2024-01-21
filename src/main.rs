use std::env::args;
use crate::rox::Rox;

fn main() {
    let mut rox = Rox{had_error: false};
    match args().count() {
        2.. => println!("Usage: rox [script]"),
        2 => rox.run_file(args().nth(1).unwrap()),
        _ => rox.run_prompt(),
    }
}
