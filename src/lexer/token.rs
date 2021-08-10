use crate::lexer::local::{Coordinate, Location};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::str::pattern::Pattern;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Copy)]
pub enum TokenType {
    StructDecl,
    EnumDecl,
    FunctionDecl,
    InterfaceDecl,
    FuncCall,
    Command,
    Literal,
    Operator,
    VariableName,
    LetDecl,
    Conditional,
    Match,
    ImportDecl,
    NewLine,
    SemiColon,
    WhiteSpace,
    MatchArm,
    NameSpaceUse,
    PublicityDecl,
    Unknown,
}

/*impl FromStr for TokenType {
    type Err = LexerError;
    fn from_str(s: &str) -> Result<Self, LexerError> {
        if s.find("struct").is_some() {
            Self::StructDecl
        } else if s.find("::").is_some() {
            Self::NameSpaceUse
        } else if s.find("enum").is_some() {
            Self::EnumDecl
        } else if s.find("pub").is_some() {
            Self::PublicityDecl
        } else if s.find("(").is_some() {
            Self::FuncCall
        } else if s.find("let").is_some() {
            Self::LetDecl
        }
    }
}*/

pub struct AxolotlToken {
    start: Coordinate,
    end: Coordinate,
    literal: String,
    kind: TokenType,
}

impl Token for AxolotlToken {
    type Matcher = &'static str;
    fn start(&self) -> &Coordinate {
        &self.start
    }

    fn end(&self) -> &Coordinate {
        &self.end
    }

    fn text(&self) -> &str {
        &self.literal
    }

    // damn string literal escapes are annoying
    fn splitters() -> Vec<Self::Matcher> {
        vec![" ", "\n", ";", "{", "}", "[", "]", "\""]
    }

    fn from_str(value: &str, start: Coordinate, end: Coordinate) -> Self {
        Self {
            literal: value.to_string(),
            start,
            end,
            kind: TokenType::StructDecl,
        }
    }
}

pub trait Token {
    type Matcher: Pattern<'static>;
    //fn create(section: String, start: Coordinate, end: Coordinate) -> Self;
    fn start(&self) -> &Coordinate;
    fn end(&self) -> &Coordinate;
    fn text(&self) -> &str;
    fn from_str(value: &str, start: Coordinate, end: Coordinate) -> Self;

    fn splitters() -> Vec<Self::Matcher>;

    fn start_line(&self) -> u32 {
        self.start().line
    }
    fn end_line(&self) -> u32 {
        self.end().line
    }
    fn start_col(&self) -> u32 {
        self.start().col
    }
    fn end_col(&self) -> u32 {
        self.end().col
    }
}

pub trait TokenStream<T: Token>:
    IntoIterator + Iterator + ParallelIterator + IntoParallelIterator + Default
{
    fn next(&self) -> Option<&T>;
    fn previous(&self) -> Option<&T>;
    fn next_mut(&mut self) -> Option<&mut T>;
    fn previous_mut(&mut self) -> Option<&mut T>;
    fn push_token(&mut self, token: T);
}
