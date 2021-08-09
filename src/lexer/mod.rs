pub mod local;
pub mod token;
use local::Location;
use token::{Token, TokenStream};

pub struct Lexer<T: Token, S: TokenStream<T>> {
    tokens: Vec<&'static str>,
    data: String,
    stream: S,
    phantom: std::marker::PhantomData<T>,
}

impl<T: Token, S: TokenStream<T>> Lexer<T, S> {
    pub fn new(data: String, tokens: Vec<&'static str>) -> Self {
        Self {
            data,
            tokens,
            stream: S::default(),
            phantom: std::marker::PhantomData::default(),
        }
    }
    pub fn into_stream(self) -> S {
        todo!()
    }
}
