use std::collections::HashMap;

use crate::types::{self, Type};

#[derive(Debug)]
pub struct SymbolTable<'a> {
    pub symbols: &'a mut HashMap<String, Type>,
    pub parent: Option<Box<&'a SymbolTable<'a>>>
}

pub trait BoxClone {
    fn clone_box(&self) -> Box<&SymbolTable>;
}

impl BoxClone for SymbolTable<'_>{
    fn clone_box(&self) -> Box<&SymbolTable> {
        Box::new(self.clone())
    }
}


impl SymbolTable<'_> {

    pub fn var_lookup(&self, dotted_name: &Vec<String>) -> Type {
        if let Some(item) = self.symbols.get(&dotted_name[0]){
            let mut current_type = item.to_owned();
            for i in 1..dotted_name.len(){
                let temp = current_type.getattr(&dotted_name[i]);
                if let types::Type::Undefined = temp {
                    break;
                }
                current_type = temp;
            }
            return current_type
        }
        Type::Undefined
    }

    pub fn clone_item(&self, key: &String) -> Type {
        let item = self.symbols.get(key);
        if let Some(i) = item {
            let clone = i.clone();
            return clone;
        }
        else if let Some(parent) = &self.parent {
            return parent.clone_item(key);
        }
        else {
            return Type::Undefined;
        }
    }
}
