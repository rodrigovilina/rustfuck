use {
  crate::{lexer::lex, parser::parse, vm::Brainfuck},
  node::Node,
  std::{
    io::{self, BufRead, Error, ErrorKind, Read},
    str::from_utf8,
  },
  token::BalancedTokens,
};

mod add;
mod comma;
mod dot;
mod left;
mod lexer;
mod node;
mod parser;
mod right;
mod sub;
mod token;
mod vm;

fn main() {
  let args: Vec<String> = std::env::args().collect();

  match args.len() {
    1 => repl(),
    2 => run_file(args),
    _ => panic!(),
  }
}

fn repl() {
  let stdin = io::stdin();
  let mut iterator = stdin.lock().lines();
  let mut vm: Brainfuck<30_000> = Brainfuck::new();
  loop {
    let code: String = iterator.next().unwrap().unwrap();
    let tokens: BalancedTokens = lex(&code);
    let nodes: Vec<Node> = parse(tokens.tokens);
    vm.run(nodes);
    vm.debug();
  }
}

fn run_file(args: Vec<String>) {
  let code: String =
    std::fs::read_to_string(&args[1]).expect("Should have been able to read the file");
  dbg!(&code);
  let tokens: BalancedTokens = lex(&code);
  dbg!(&tokens);
  let nodes: Vec<Node> = parse(tokens.tokens);
  dbg!(&nodes);
  let mut vm: Brainfuck<30_000> = Brainfuck::new();

  vm.run(nodes);
  vm.debug();
}

pub fn read_char_from_stdin() -> char {
  let mut buffer = [0; 4];
  let mut stdin = io::stdin();

  stdin.read_exact(&mut buffer[0..1]).unwrap();
  let mut len = 1;
  if from_utf8(&buffer[0..len]).is_err() {
    while len < 4 && (buffer[len - 1] & 0b11000000) == 0b10000000 {
      stdin.read_exact(&mut buffer[len..len + 1]).unwrap();
      len += 1;
    }
  }

  from_utf8(&buffer[0..len])
    .unwrap()
    .chars()
    .next()
    .ok_or(Error::new(ErrorKind::UnexpectedEof, "No character found"))
    .unwrap()
}
