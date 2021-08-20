use crate::lexer::Token;
use crate::*;
pub trait Parser<'a> {
    type ParsedOutput;
    type Err: std::error::Error = ParseError;
    fn from_tokens(tokens: Vec<&'a Token<'a>>) -> Self;
    fn parse(&self) -> Result<(u64, Self::ParsedOutput), Self::Err>;
}

pub struct InterfaceParser<'a> {
    tokens: Vec<&'a Token<'a>>,
}

impl<'a> Parser<'a> for InterfaceParser<'a> {
    type ParsedOutput = crate::InterfaceRef;
    fn from_tokens(tokens: Vec<&'a Token<'a>>) -> Self {
        Self { tokens }
    }
    fn parse(&self) -> Result<(u64, InterfaceRef), Self::Err> {
        todo!();
    }
}

pub struct StructParser<'a> {
    tokens: Vec<&'a Token<'a>>,
}

impl<'a> Parser<'a> for StructParser<'a> {
    type ParsedOutput = crate::StructRef;
    fn from_tokens(tokens: Vec<&'a Token<'a>>) -> Self {
        Self { tokens }
    }
    fn parse(&self) -> Result<(u64, StructRef), Self::Err> {
        todo!();
    }
}

pub struct EnumParser<'a> {
    tokens: Vec<&'a Token<'a>>,
}

impl<'a> Parser<'a> for EnumParser<'a> {
    type ParsedOutput = crate::EnumRef;
    fn from_tokens(tokens: Vec<&'a Token<'a>>) -> Self {
        Self { tokens }
    }
    fn parse(&self) -> Result<(u64, EnumRef), Self::Err> {
        todo!();
    }
}
