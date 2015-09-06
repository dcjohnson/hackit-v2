extern crate libhackit_v2;
use libhackit_v2::lex::{Lexer, LexerTrait};
use libhackit_v2::parse::{Parser, ParserTrait};
use libhackit_v2::eval::{Eval, EvalTrait, PrettyPrint};

fn main() {
    // let lexer = Lexer::new("(123 (abc) <123 \"eab \" \" 34.52b \" abc \" 234\" -123 abc.3 >)");
    // let lexer = Lexer::new("(<<><<>>><><><<><<><>>><>) (set v <1 2 3 (add 3 (mut 3 5) -2.43)>) (def abc (e f y) ((set v 3) (set v (add (add 3 (sub -0.342 (mult 39 423)) 6 3) 2 e (add 3 (sub 5 (mult -2 (add 1.7 52) 7) 1) 5))) (add e f y v))) (print (abc 1 2 3))");
    let lexer = Lexer::new("(print 3 abvc \"3an3\") (print \"ersan\")");
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
    // println!("{}", eval.pretty_print());
}
