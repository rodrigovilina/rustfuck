#![deny(clippy::complexity)]
#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![deny(clippy::perf)]
#![deny(clippy::empty_structs_with_brackets)]
#![deny(clippy::min_ident_chars)]
#![deny(clippy::panic)]
#![deny(clippy::expect_used)]
#![deny(clippy::unwrap_used)]

mod add;
mod balanced_tokens;
mod comma;
mod dot;
mod left;
mod lexer;
mod node;
mod parser;
mod repl;
mod right;
mod runner;
mod sub;
mod token;
mod vm;

fn main() {
  let args: Vec<String> = std::env::args().collect();

  match args.len() {
    1 => repl::repl(),
    2 => runner::run_file(args),
    _ => panic!(),
  }
}
