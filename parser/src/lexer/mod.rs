use logos::Logos;

pub struct Location {
    filename: String,
    line: u16,
    col: u16,
}

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum TokenType<'a> {
    #[token(":")]
    TypeAnnotation,
    #[token("::")]
    NameSpaceOp,
    #[token("{")]
    StartBlock,
    #[token("}")]
    EndBlock,
    #[token(" ")]
    Space,
    #[token("||")]
    LogicalOr,
    #[token("|")]
    Pipe,
    #[token("&&")]
    LogicalAnd,
    #[token(";")]
    SemiColon,
    #[token("struct")]
    StructDecl,
    #[token("enum")]
    EnumDecl,
    #[token("interface")]
    InterfaceDecl,
    #[token("fn")]
    FnDecl,
    #[token("==")]
    EqualityOp,
    #[token("<=")]
    LessOrEq,
    #[token(">=")]
    GreatOrEq,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("=")]
    Assignment,
    #[regex("[a-zA-Z]+")]
    Text(&'a str),
    #[regex("[0-9]+")]
    Numeric,
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    WhiteSpace,
    #[token("//")]
    OneLineComment,
    #[token("///")]
    DocComment,
    #[token("/*")]
    MultiLineStart,
    #[token("*/")]
    MultiLineEnd,
    #[token("/*!")]
    MultiLineDocStart,
    #[token("!*/")]
    MultiLineDocEnd,
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("&")]
    AndSymbol,
    #[token(",")]
    Comma,
    #[token("#")]
    HashTag,
    #[token("[")]
    OpenBracket,
    #[token("]")]
    CloseBracket,
}

#[test]
fn lex_file() {
    let value = include_str!("example.axol");
    println!("value: {}", value);
    let mut lexer = TokenType::lexer(value);
    for token in lexer {
        println!("TokenType: {:?}", token);
    }
    panic!();
}