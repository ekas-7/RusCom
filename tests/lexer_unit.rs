use ruscom::lexer::token::Token;
use ruscom::lexer::Lexer;

#[test]
fn simple_ident_and_number() {
    let src = "int x = 42;";
    let mut lex = Lexer::new(src);
    assert_eq!(lex.next().unwrap().unwrap(), Token::Identifier("int".into()));
    assert_eq!(lex.next().unwrap().unwrap(), Token::Identifier("x".into()));
    assert_eq!(lex.next().unwrap().unwrap(), Token::Operator("=".into()));
    assert_eq!(lex.next().unwrap().unwrap(), Token::Number("42".into()));
    assert_eq!(lex.next().unwrap().unwrap(), Token::Punct(';'));
}

#[test]
fn comments_and_whitespace() {
    let src = "// line comment\n/* block */\nfoo";
    let mut lex = Lexer::new(src);
    assert_eq!(lex.next().unwrap().unwrap(), Token::Identifier("foo".into()));
}
