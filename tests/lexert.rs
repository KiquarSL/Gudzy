use guidzy::lexer::Lexer;

#[test]
fn main() {
    let tokens_str = "
+ - * / ! () {} , = && || < > <= >= == !=
\"my string\"
my_ident12 true false print
3.14 45 100_000
"
    .trim();
    let tokens = Lexer::new(tokens_str).tokenize();
    println!("Source: {}\nTokens:", tokens_str);
    for tkn in tokens {
        println!("{:?}", tkn);
    }
}
