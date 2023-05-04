use crate::{symbol_table::SymbolTable, types};
use text_io::read;

macro_rules! builtin_func {
    ($name:ident($env:ident, $params:ident) $func:block) => {
        pub fn $name($env: &mut SymbolTable, $params: Vec<Box<types::Type>>) -> types::Type $func
    };
}

builtin_func! {
    print(env, _params) {
        if let Some(types::Type::List(params)) = env.symbols.get("ARGV") {
            for param in params {
                let data = param.into_string();
                print!("{}", data);
            }
        }
        types::Type::Int(0)
    }
}

builtin_func! {
    println(env, _params){
        if let Some(types::Type::List(params)) = env.symbols.get("ARGV"){
            for param in params {
                let data = param.into_string();
                print!("{}", data);
            }
        }
        println!();
        types::Type::Int(0)
    }
}

builtin_func! {
    set(env, _params){
        if let Some(types::Type::StructInstance { name, mut fields }) = env.symbols.remove("str") {
            if let Some(types::Type::String(x)) = env.symbols.get("name") {
                if let Some(t) = env.symbols.get("val") {
                    fields.insert(x.to_owned(), t.to_owned());
                    return types::Type::StructInstance {
                        name: name.to_owned(),
                        fields: fields.to_owned(),
                    };
                }
            }
        }
        panic!("Cannot use set on non-struct")
    }
}

builtin_func!{
    repr(_env, params) {
        types::Type::String(params[0].into_repr())
    }
}

builtin_func!{
    input(_env, _params) {
        let line = read!("{}\n");
        types::Type::String(line)
    }
}
