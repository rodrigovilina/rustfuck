use crate::{
  balanced_tokens::BalancedTokens, lexer::lex, node::Node, parser::parse, vm::Brainfuck,
};

pub fn run_file(args: Vec<String>) {
  let code: String =
    std::fs::read_to_string(&args[1]).expect("Should have been able to read the file");
  dbg!(&code);

  let tokens: BalancedTokens = lex(&code);
  dbg!(&tokens);

  let nodes: Vec<Node> = parse(tokens.tokens);
  dbg!(&nodes);

  let mut vm: Brainfuck<30_000> = Brainfuck::new();
  vm.run(&nodes);
  vm.debug();
}
