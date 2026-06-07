use guidzy::lexer::Lexer;
use guidzy::parser::Parser;

#[test]
fn main() {
    let source = "
2 + 2 * 2 > 4 / 2 + 2 && true != false
(2 + 2) * 2
"
    .trim();
    let tokens = Lexer::new(source).tokenize();
    println!("Source: {}\nTokens:", source);
    for tkn in &tokens {
        println!("{:?}", tkn);
    }
    let exprs = Parser::new(tokens, source).parse_exprs();
    println!("Expressions:");
    for ex in exprs {
        println!("{}", ex);
    }
}
