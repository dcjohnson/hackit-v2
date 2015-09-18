extern crate libhackit_v2;
use libhackit_v2::lex::{Lexer, LexerTrait};
use libhackit_v2::parse::{Parser, ParserTrait};
use libhackit_v2::eval::{Eval, EvalTrait};

fn main() {
    let lexer = Lexer::new("((set (name a) (params b c) (body (println (add 5 (add (c) (b)) 20))))(a 90 1))");
    // let lexer = Lexer::new("((set (name a) (params) (body (println (add 3 (add 3 4) 20))))(a))");
    let mut parser = Parser::new();
    let mut eval = Eval::new_option(match lexer.lex() {
        Some(tokens) => {
            for tok in tokens.into_iter() {
                if !parser.parse_token(tok) {
                    break;
                }
            }
            parser.get_parsed_tree()
        },
        None => None
    });
    eval.eval();
}
