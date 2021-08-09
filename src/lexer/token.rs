use crate::lexer::local::{Location, Coordinate};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::str::pattern::Pattern;

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
}

pub struct AxolotlToken {
    start: Coordinate,
    end: Coordinate,
    literal: String,
    kind: TokenType,
}

impl Token for AxolotlToken {
    fn start(&self) -> &Coordinate {
        &self.start
    }

    fn end(&self) -> &Coordinate {
        &self.end
    }

    fn text(&self) -> &str {
        &self.literal
    }

    fn splitters() -> Vec<char> {
        vec![' ', '\n', ':', ';', '{', '}', '[', ']', ',', '(', ')', '\"']
    }

    fn from_str(value: &str, start: Coordinate, end: Coordinate) -> Self {
        Self {
            literal: value.to_string(),
            start,
            end,
            kind: TokenType::Literal,
        }
    }
}

pub trait Token {
    //fn create(section: String, start: Coordinate, end: Coordinate) -> Self;
    fn start(&self) -> &Coordinate;
    fn end(&self) -> &Coordinate;
    fn text(&self) -> &str;
    fn from_str(value: &str, start: Coordinate, end: Coordinate) -> Self;
    //fn starts_with() -> Self::StartPattern;
    //fn ends_with() -> Self::EndPattern;

    fn splitters() -> Vec<char>;

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
