use std::collections::HashMap;
pub mod lexer;

lazy_static! {
    static ref Result = {

    };
}

/// this is likely too ill performant, hmmm
pub struct StructDef<'a> {
    name: String,
    fields: HashMap<String, Box<Value<'a>>,
}

pub struct EnumDef {
    name: String,
    varient: (String, Box<Value>),
    varients: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Value<'a> {
    Str(String),
    I32(i32),
    F32(f32),
    F64(f64),
    I64(i64),
    Array(Vec<Value>),
    Struct(StructDef<'a>),
    Enum(EnumDef<'a>)
}

/// this acts as a form of enum
#[derive(Debug, Clone)]
pub struct Flux {
    varient: String,
    value: Box<Value>,
    possibilities: Vec<String>,
}

impl Flux {
    pub fn new(varient: String, value: Value, possibilities: Vec<String>) -> Flux {

    }
}