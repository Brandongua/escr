use crate::symbol_table::SymbolTable;
use crate::util::{String_vec,str};
use crate::{builtin_functions, nodes, types};
// use crate::tokens::TT;
use crate::types::Type;

macro_rules! insert_func {
    ($name:ident ($($p:literal$(,)?)*) { $exec:expr } into $symbols:ident) => {
        let name = stringify!($name);
        $symbols.symbols.insert(
            name.to_string(),
            types::Type::BuiltinFunction {
                name: name.to_string(),
                parameters: vec![$( $p.to_owned(), )*],
                code: $exec
            }
        )
    };
}

pub fn interpret<'a>(program: nodes::Node, symbols: &mut SymbolTable) -> Type {
    insert_func!(
        print("...text"){
            builtin_functions::print
        } into symbols
    );
    insert_func!(
        input(){
            builtin_functions::input
        } into symbols
    );
    insert_func!(
        println("...text"){
            builtin_functions::println
        } into symbols
    );
    insert_func!(
        repr("text"){
            builtin_functions::repr
        } into symbols
    );
    insert_func!(
        set("str", "name", "val"){
            builtin_functions::set
        } into symbols
    );
    program.visit(symbols)
}
