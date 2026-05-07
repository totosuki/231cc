use std::env;
use std::process;

use rcc::node::*;
use rcc::parser::*;
use rcc::tokenizer::*;

fn generate(node: Node) {
    if node.kind == NodeKind::Num {
        println!("  push {}", node.val.unwrap());
        return;
    }

    generate(*node.lhs.unwrap());
    generate(*node.rhs.unwrap());

    println!("  pop rdi");
    println!("  pop rax");

    match node.kind {
        NodeKind::Add => {
            println!("  add rax, rdi");
        }
        NodeKind::Sub => {
            println!("  sub rax, rdi");
        }
        NodeKind::Mul => {
            println!("  imul rax, rdi");
        }
        NodeKind::Div => {
            println!("  cqo");
            println!("  idiv rdi");
        }
        NodeKind::EQ => {
            println!("  cmp rax, rdi");
            println!("  sete al");
            println!("  movzb rax, al");
        }
        NodeKind::NE => {
            println!("  cmp rax, rdi");
            println!("  setne al");
            println!("  movzb rax, al");
        }
        NodeKind::LT => {
            println!("  cmp rax, rdi");
            println!("  setl al");
            println!("  movzb rax, al");
        }
        NodeKind::LE => {
            println!("  cmp rax, rdi");
            println!("  setle al");
            println!("  movzb rax, al");
        }
        _ => (),
    }

    println!("  push rax");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        process::exit(1);
    }

    let mut tokenizer: Tokenizer = Tokenizer::new(vec![], 0, args[1].clone());
    tokenizer.tokenize(args[1].clone());
    let mut parser: Parser = Parser::new(tokenizer);
    let node: Node = parser.expr();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    generate(node);

    println!("  pop rax");
    println!("  ret");
}
