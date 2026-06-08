use guidzy::lexer::Lexer;
use guidzy::parser::Parser;

#[test]
fn main() {
    expression();
    statement();
}

fn expression() {
    let source = "
2 + 2 * 2 > 4 / 2 + 2 && true != false
(2 + 2) * 2
"
    .trim();
    let tokens = Lexer::new(source).tokenize();
    println!("Source: {}", source);
    let exprs = Parser::new(tokens, source).parse_exprs();
    println!("Expressions:");
    for ex in exprs {
        println!("{}", ex);
    }
}

fn statement() {
    let source = "
a = 4
print(\"a is \", a)
"
    .trim();
    let tokens = Lexer::new(source).tokenize();
    println!("Source: {}", source);
    let stmts = Parser::new(tokens, source).parse();
    println!("Statements:");
    for stmt in stmts {
        println!("{:?}", stmt);
    }
}
