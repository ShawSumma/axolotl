use logos::Logos;
mod location;
pub use location::*;
use std::collections::LinkedList;
use std::convert::TryInto;

/// NOTE: because of an issue with logos not supporting ranges from regex
/// more then 3 empty lines will cause weird behavior with source tracing
#[derive(Logos, Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum TokenType {
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
    #[token("func")]
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
    Text,
    #[regex("[0-9]+")]
    Numeric,
    #[regex("[\n\r]+")]
    NewLine,
    //#[regex("/(\n){2,}/|/(\r){2,}/")]
    #[regex("\n\n|\r\r")]
    EmptyLine,
    #[regex("\n\n\n|\r\r\r")]
    TwoEmpty,
    #[regex("\n\n\n\n|\r\r\r\r")]
    ThreeEmpty,
    #[error]
    #[regex(r"[ \t\f]+")]
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
    #[token("$(")]
    CommandRedirect,
    #[token("$")]
    VariableAccess,
    #[token("->")]
    RetIndicator,
    #[token("#!")]
    ShellDirective,
    #[token("impl")]
    ImplementationDecl,
    #[token("if")]
    BeginIf,
    #[token("loop")]
    LoopDecl,
    #[token("break")]
    BreakKey,
    #[token("else")]
    ElseDecl,
    #[token("import")]
    ImportDecl,
    #[token("for")]
    ForLoop,
    #[token("..")]
    Range,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Token<'a> {
    pub kind: TokenType,
    pub start: Coordinate,
    pub end: Coordinate,
    pub literal: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenType, start: Coordinate, end: Coordinate, literal: &'a str) -> Self {
        Self {
            kind,
            start,
            end,
            literal,
        }
    }
}

#[derive(Debug)[]
pub struct AxolotlLexer<'a> {
    literal: &'a str,
    filename: String,
    tokens: LinkedList<Token<'a>>,
}

impl<'a> AxolotlLexer<'a> {
    /// # Arguments
    /// literal: the data to be lexed,
    /// filename: the name of the source file that the data comes from
    pub fn new(literal: &'a str, filename: String) -> Self {
        let tempself = Self {
            literal,
            filename,
            tokens: LinkedList::new(),
        };
        tempself.create_tokens(TokenType::lexer(literal))
    }
    fn create_tokens(mut self, lexer: logos::Lexer<'a, TokenType>) -> Self {
        let mut start: Coordinate = Coordinate {line: 1, col: 0};
        //let mut list = LinkedList::new();
        for (token, span) in lexer.spanned() {
            let end = Coordinate{ line: start.line, col: start.col + (span.end - span.start)};
            self.tokens.push_back(Token::new(
                token,
                start,
                end,
                &self.literal[span.clone()]
            ));
            if token == TokenType::NewLine{
                start = start.newline(1);
            }else if token == TokenType::EmptyLine {
                start = start.newline(2);
            }else if token == TokenType::TwoEmpty {
                start = start.newline(3);
            }else if token == TokenType::ThreeEmpty {
                start = start.newline(4);
            }
            else{
                start += (span.end - span.start);
            }
        }
        self
    }
    pub fn iter(&self) -> std::collections::linked_list::Iter<'_, Token<'_>> {
        self.tokens.iter()
    }
}

#[test]
fn lex_file() {
    let value = include_str!("example.axol");
    let lexer = AxolotlLexer::new(value, String::from("example.axol"));
    for token in lexer.iter() {
        println!("token: {:?}", token);
    }
    panic!();
}
