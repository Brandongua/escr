mod interpreter;
mod lexer;
mod nodes;
mod parser;
mod symbol_table;
mod tokens;
mod types;
mod builtin_functions;

mod util;


// use nodes::Node;

use std::{collections::HashMap, env};

use symbol_table::SymbolTable;


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut text = String::from("");
    let mut found_text = false;
    if args.len() == 2 {
        if std::path::Path::new(&args[1]).exists() {
            match std::fs::read_to_string(&args[1]) {
                Ok(t) => text = t,
                _ => panic!("Could not read {}", args[1]),
            }
            found_text = true;
        }
    }
    if !found_text {
        for i in 1..args.len() {
            text += &args[i];
        }
    }
    // println!("{}", text);
    let mut l = lexer::Lexer::new(text);
    let toks = l.parse();
    // println!("{:?}", toks);
    let mut p = parser::Parser::new(toks);
    let nodes = p.parse();
    // let mut int = interpreter::Interpreter::new();
    // int.interpret(nodes);
    // println!("{}", nodes.repr(0));
    let mut symbols: HashMap<String, types::Type> = HashMap::new();
    let _res = interpreter::interpret(
        nodes,
        &mut SymbolTable {
            symbols: &mut symbols,
            parent: None,
        },
    );
}
