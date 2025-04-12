use {
  crate::{balanced_tokens::BalancedTokens, lexer::lex, node::Node, parser::parse, vm::Brainfuck},
  std::io::{self, BufRead},
};

pub fn repl() {
  let stdin = io::stdin();
  let mut iterator = stdin.lock().lines();
  let mut vm: Brainfuck<30_000> = Brainfuck::new();
  loop {
    let code: String = iterator.next().unwrap().unwrap();
    let tokens: BalancedTokens = lex(&code);
    let nodes: Vec<Node> = parse(tokens.tokens);
    vm.run(&nodes);
    vm.debug();
  }
}
