#![feature(pattern)]
#![feature(linked_list_cursors)]
#![feature(associated_type_defaults)]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate err_derive;
pub mod lexer;
mod parsers;
use crate::parsers::Parser;
use lexer::*;
pub use parsers::*;
pub mod tree;

use crate::lexer::Location;
use std::collections::HashMap;
use std::collections::{linked_list::Cursor, LinkedList};
use std::string::ToString;

pub enum Mutability {
    Mutable,
    Immutable,
}

#[derive(Debug, Clone, Error)]
pub enum ParseError {
    #[error(display = "missing , at {}", _0)]
    MissingComma(String),
    #[error(display = "missing ending block for struct starting at: {}", _0)]
    MissingEndBlock(Coordinate),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructRef {
    name: String,
    fields: HashMap<String, Value>,
    funcs: HashMap<String, FunctionRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumRef {
    name: String,
    varient: (String, Value),
    varients: Vec<(String, Value)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Publicity {
    Public,
    Package,
    Scope,
    Private,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    kind: Type,
    publicity: Publicity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceRef {
    name: String,
    functions: FunctionRef,
    assoctypes: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generic {
    name: String,
    constraints: Vec<InterfaceRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    generics: Vec<Generic>,
    ret: Value,
    args: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionRef {
    name: String,
    sig: Signature,
}

pub struct Reference {
    mutable: Mutability,
    val: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Str,
    Array,
    Struct(String),
    Enum(String),
    /// this u64 should identify a type signature
    Func(String),
    Alias((String, String)),
    // will be parsed seperately
    Reference(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    name: String,
    kind: Type,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operation {
    Assignment,
    Add,
    Subtract,
    Multiply,
    Divide,
    Index,
    Call((String, Vec<Value>)),
}

pub struct AxolotlParser<'a> {
    filename: String,
    literal: &'a str,
    structs: HashMap<u64, StructRef>,
    enums: HashMap<u64, EnumRef>,
    functions: HashMap<u64, FunctionRef>,
    // holds type information for each typeid
    types: HashMap<u64, Value>,
    // maps typename to typeid
    type_maps: HashMap<String, u64>,
    interfaces: HashMap<u64, InterfaceRef>,
}

impl<'a> AxolotlParser<'a> {
    pub fn parse(&mut self, tokens: Vec<Token<'a>>) {
        if tokens.len() == 0 {
            panic!("tokens are empty");
        }

        for token in tokens.iter() {}
    }
    /*fn parse_types<'b>(&mut self, cursor: &mut Cursor<'b, Token<'a>>, token: &Token<'a>) -> Result<(), ParseError>{
        match token.kind {
            TokenType::StructDecl => {
                let tokens = match Self::find_next(cursor, TokenType::EndBlock) {
                    Some(val) => val,
                    None => return Err(ParseError::MissingEndBlock(token.start)),
                };
                let parser = StructParser::from_tokens(tokens);
                let (id, struct_val) = parser.parse()?;
                self.structs.insert(id, struct_val);
            },
            // hmm, func decl will be hard, same with if, else, etc because tree will need to be built
            TokenType::InterfaceDecl => {
                let tokens = match Self::find_next(cursor, TokenType::EndBlock) {
                    Some(val) => val,
                    None => return Err(ParseError::MissingEndBlock(token.start)),
                };
                let parser = InterfaceParser::from_tokens(tokens);
                let (id, interface) = parser.parse()?;
                self.interfaces.insert(id, interface);
            },
            TokenType::EnumDecl => {
                let tokens = match Self::find_next(cursor, TokenType::EndBlock) {
                    Some(val) => val,
                    None => return Err(ParseError::MissingEndBlock(token.start)),
                };
                let parser = EnumParser::from_tokens(tokens);
                let (id, enum_val) = parser.parse()?;
                self.enums.insert(id, enum_val);
            }
            _ => unimplemented!(),
        }
        Ok(())
    }*/

    /*fn find_next<'b>(cursor: &mut Cursor<'b, Token<'a>>, token: TokenType) -> Option<Vec<&'b Token<'a>>> {
        let mut distance = 0;
        let mut retval = Vec::new();
        let mut found = false;
        while let Some(peek) = cursor.peek_next() {
            retval.push(peek);
            found = true;
            if peek.kind == token {
                break;
            }
            distance += 1;
            cursor.move_next();
        }
        while distance != 0 {
            cursor.move_prev();
            distance -= 1;
        }
        if found {
            Some(retval)
        }else{
            None
        }
    }*/

    pub fn new(filename: String, literal: &'a str) -> Self {
        Self {
            filename,
            literal,
            structs: HashMap::new(),
            enums: HashMap::new(),
            functions: HashMap::new(),
            types: HashMap::new(),
            type_maps: HashMap::new(),
            interfaces: HashMap::new(),
        }
    }
}
