use std::collections::HashMap;
use std::string::ToString;
use crate::lexer::local::Location;

#[derive(Debug, Clone, Error)]
pub enum ParseError {
    #[error(display = "missing , at {}", _0)]
    MissingComma(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructRef {
    name: String,
    fields: HashMap<String, Value>,
    funcs: HashMap<String, FunctionRef>
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    Str,
    I32,
    F32,
    F64,
    I64,
    Array,
    Struct(u64),
    Enum(u64),
    /// this u64 should identify a type signature
    Func(u64),
    Alias((String, u64)),
    Reference(u64),
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

pub struct Parser {
    file: String,
    structs: HashMap<u64, StructRef>,
    enums: HashMap<u64, EnumRef>,
    functions: HashMap<u64, FunctionRef>,
    // holds type information for each typeid
    types: HashMap<u64, Value>,
    // maps typename to typeid
    type_maps: HashMap<String, u64>,
}

pub struct TokenStream;

impl Parser {
    pub fn parse_structt(&self, start: &Location, s: &str) -> Result<StructRef, ParseError> {
        todo!()
    }
    pub fn parse_enum(s: &str) -> Result<EnumRef, ParseError> {
        todo!()
    }
    /// returns an error if either '{' or '}' isn't found
    /// returns the error with the string inside, but should return a Location
    /*fn isolate_block(&self, s: &str, start: &Location) -> Result<String, ParseError> {
        let start = if let Some(start) = s.find('{') {
            start
        }else{
            return Err(ParseError::MissingCurly(s.to_string()));
        };
        let end = if let Some(end) = s.find('}') {

        }else{
            return Err(ParseError::MissingCloseCurly(s.to_string()))
        };
    }*/

    pub fn parse(&mut self) -> Result<TokenStream, ParseError> {


        Ok(TokenStream)
    }

    pub fn new(data: String) -> Self {
        Self {
            file: data,
            structs: HashMap::new(),
            enums: HashMap::new(),
            functions: HashMap::new(),
        }
    }
}