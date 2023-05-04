use core::fmt;

use std::{collections::HashMap, f64::NAN};

use crate::{
    nodes::{self, Node},
    symbol_table::SymbolTable,
};

pub enum Number {
    Int(i64),
    Float(f64),
}

pub trait BoxClone {
    fn clone_box(&self) -> Box<Type>;
}

#[derive(derivative::Derivative)]
#[derivative(Debug, Clone)]
pub enum Type {
    Float(f64),
    Int(i64),
    String(String),
    Undefined,
    List(Vec<Box<Type>>),
    Node(nodes::Node),
    Struct{
        name: String,
        fields: Vec<String>
    },
    StructInstance {
        name: String,
        fields: HashMap<String, Type>
    },
    Function {
        name: String,
        parameters: Vec<String>,
        code: Node,
    },
    BuiltinFunction {
        name: String,
        parameters: Vec<String>,
        #[derivative(Debug="ignore")]
        code: fn(env: &mut SymbolTable, params: Vec<Box<Type>>) -> Type
    },
}


macro_rules! bin_op_numbers {
    ($left:ident $op:tt $right:ident) => {
        match $left {
            Number::Int(x) => match $right {
                Number::Int(y) => Type::Int(x $op y),
                Number::Float(y) => Type::Int(x $op y as i64),
            },
            Number::Float(x) => match $right {
                Number::Float(y) => Type::Float(x $op y),
                Number::Int(y) => Type::Float(x $op y as f64),
            },
        }
    };
}

impl Type {
    pub fn access(&self) -> Type {
        return self.to_owned();
    }
    pub fn setattr(&mut self, name: &String, value: Type) -> Type {
        match self {
            Type::StructInstance{fields, ..} => {
                if let Some(..) = fields.get(name){
                    fields.insert(name.to_owned(), value.to_owned());
                    return value;
                }
                Type::Undefined
            }
            _ => Type::Undefined
        }
    }
    pub fn getattr(&self, name: &String) -> Type {
        match self {
            Type::StructInstance{fields, ..} => {
                if let Some(item) = fields.get(name){
                    return item.to_owned();
                }
                Type::Undefined
            }
            _ => Type::Undefined
        }
    }

    pub fn sub(&self, other: &Type) -> Type {
        let left = self.into_number();
        let right = other.into_number();

        bin_op_numbers!(left - right)
    }
    pub fn add(&self, other: &Type) -> Type {
        let left = self.into_number();
        let right = other.into_number();

        bin_op_numbers!(left + right)
    }

    pub fn mul(&self, other: &Type) -> Type {
        let left = self.into_number();
        let right = other.into_number();
        bin_op_numbers!(left * right)
    }

    pub fn div(&self, other: &Type) -> Type {
        let left = self.into_number();
        let right = other.into_number();
        bin_op_numbers!(left / right)
    }

    pub fn dot_access(&self, name: String) -> Type {
        match self.access() {
            Type::StructInstance{fields, ..} => {
                if let Some(val) = fields.get(&name.to_owned()){
                    return val.to_owned();
                }
                return Type::Undefined
            }
            Type::String(x) => {
                if name == "length" {
                    return Type::Int(x.len() as i64);
                }
                Type::Undefined
            }
            Type::List(x) => {
                if name == "length" {
                    return Type::Int(x.len() as i64)
                }
                Type::Undefined
            }
            _ => Type::Undefined
        }
    }

    pub fn into_number(&self) -> Number {
        match self.access() {
            Type::Int(n) => Number::Int(n.to_owned()),
            Type::Float(n) => Number::Float(n.to_owned()),
            Type::String(x) => Number::Int(x.len() as i64),
            Type::Function { .. } | Type::BuiltinFunction { .. } | Type::Node(..) => Number::Int(0),
            Type::Undefined => Number::Float(NAN),
            Type::List(n) => Number::Int(n.len() as i64),
            Type::Struct { .. } => Number::Int(0),
            Type::StructInstance{fields, ..} => {
                if let Some(Type::Function{name: _name, parameters: _params, code: _code}) = fields.get("into_number") {
                    Number::Int(1)
                }
                else {
                    Number::Int(0)
                }
            }
        }
    }

    pub fn into_repr(&self) -> String {
        match self {
            Type::Int(x) => x.to_string(),
            Type::Float(x) => x.to_string(),
            Type::List(..) => self.into_string(),
            Type::Undefined => "[[undefined]]".to_string(),
            Type::Node(n) => n.repr(0),
            Type::BuiltinFunction { .. } => "[[builtin function]]".to_string(),
            Type::Struct { name, .. } => "[[".to_owned() + name + "]]",
            Type::StructInstance { name, .. } => "[[".to_owned() + name + "()]]",
            Type::Function {
                name, parameters, ..
            } => {
                let mut text = String::from("[[function]] ") + &name;
                text += &String::from("(");
                for i in 0..parameters.len() {
                    text += &parameters[i].to_string();
                    if i < parameters.len() - 1 {
                        text += &String::from(", ");
                    }
                }
                text += &String::from(")");
                return text;
            }
            Type::String(s) => "\"".to_owned() + s + "\"",
        }
    }

    pub fn into_string(&self) -> String {
        match self.access() {
            Type::Int(x) => x.to_string(),
            Type::Float(x) => x.to_string(),
            Type::String(x) => x,
            Type::Function { .. } => "[[function]]".to_string(),
            Type::BuiltinFunction { .. } => "[[builtin function]]".to_string(),
            Type::Undefined => "undefined".to_string(),
            Type::Node(..) => "[[node]]".to_owned(),
            Type::Struct { name, .. } => name.to_owned(),
            Type::StructInstance { name, .. } => name + "()",
            Type::List(n) => {
                let mut text = String::from("[");
                for i in 0..n.len() {
                    text += &n[i].into_repr();
                    if i < n.len() - 1 {
                        text += &String::from(", ");
                    }
                }
                text += "]";
                return text;
            }
        }
    }

    pub fn run(&self, given_params: Vec<Type>, env: &mut SymbolTable) -> Type {
        match self {
            Type::Node(n) => n.visit(env),
            Type::Struct {name, fields} => {
                let mut map = HashMap::new();
                let mut i = 0;
                for field in fields {
                    if i >= given_params.len() {
                        break;
                    }
                    map.insert(field.to_owned(), given_params[i].clone());
                    i += 1;
                }
                return Type::StructInstance{name: name.to_owned(), fields: map}
            }
            Type::BuiltinFunction { name, parameters, code } => {
                let mut symbols: HashMap<String, Type> = HashMap::new();
                let mut args = vec![];
                //we pick the smallest so that if they dont provide exactly the right amount of args, it
                //doesn't error.
                for i in 0..std::cmp::min(given_params.len(), parameters.len()) {
                    symbols.insert(parameters[i].clone(), given_params[i].clone());
                    args.push(Box::new(given_params[i].clone()));
                }
                for i in std::cmp::min(given_params.len(), parameters.len())..given_params.len() {
                    args.push(Box::new(given_params[i].clone()));
                }

                symbols.insert("ARGV".to_owned(), Type::List(args.clone()));
                let mut child_table = SymbolTable {
                    parent: Some(Box::new(env)),
                    symbols: &mut symbols,
                };
                let data = code(&mut child_table, args);
                return data;
            }
            Type::Function {
                name: _name,
                parameters,
                code,
            } => {
                let mut symbols: HashMap<String, Type> = HashMap::new();
                let mut args = vec![];
                //we pick the smallest so that if they dont provide exactly the right amount of args, it
                //doesn't error.
                for i in 0..std::cmp::min(given_params.len(), parameters.len()) {
                    symbols.insert(parameters[i].clone(), given_params[i].clone());
                    args.push(Box::new(given_params[i].clone()));
                }
                for i in std::cmp::min(given_params.len(), parameters.len())..given_params.len() {
                    args.push(Box::new(given_params[i].clone()));
                }
                symbols.insert("ARGV".to_owned(), Type::List(args));
                let mut child_table = SymbolTable {
                    parent: Some(Box::new(env)),
                    symbols: &mut symbols,
                };
                let data = code.visit(&mut child_table);
                return data;
            }
            _ => panic!("Cannot call {:?}", self),
        }
    }
}
