extern crate libhackit_v2;
use libhackit_v2::lex::{Lexer, LexerTrait};
use libhackit_v2::parse::{Parser, ParserTrait};
use libhackit_v2::eval::{Eval, EvalTrait, PrettyPrint};

fn main() {
    let lexer = Lexer::new("((set a (params b) (println (add -3.1 4 2))) (a 3))");
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
