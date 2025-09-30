use ruscom::lexer::token::Token;
use ruscom::lexer::Lexer;

#[test]
fn simple_ident_and_number() {
    let src = "int x = 42;";
    let mut lex = Lexer::new(src);
    // collect and log tokens
    let mut tokens = Vec::new();
    while let Some(r) = lex.next() {
        let t = r.unwrap();
        if t == Token::Eof { break; }
        tokens.push(t);
    }
    eprintln!("simple_ident_and_number tokens ({}): {:?}", tokens.len(), tokens);
    assert_eq!(tokens[0], Token::Identifier("int".into()));
    assert_eq!(tokens[1], Token::Identifier("x".into()));
    assert_eq!(tokens[2], Token::Operator("=".into()));
    assert_eq!(tokens[3], Token::Number("42".into()));
    assert_eq!(tokens[4], Token::Punct(';'));
}

#[test]
fn comments_and_whitespace() {
    let src = "// line comment\n/* block */\nfoo";
    let mut lex = Lexer::new(src);
    let mut tokens = Vec::new();
    while let Some(r) = lex.next() {
        let t = r.unwrap();
        if t == Token::Eof { break; }
        tokens.push(t);
    }
    eprintln!("comments_and_whitespace tokens ({}): {:?}", tokens.len(), tokens);
    assert_eq!(tokens[0], Token::Identifier("foo".into()));
}
