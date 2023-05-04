use std::collections::HashMap;
use std::ops::Deref;

use crate::{lexer, parser};
use crate::symbol_table::SymbolTable;
use crate::tokens::TT;
use crate::types::{Number, Type};

use crate::util::ternary;

macro_rules! compare_numbertype {
    ($left:ident $op:tt $right:ident) => {
        match $left.into_number() {
               Number::Int(l) => {
                   match $right.into_number() {
                       Number::Int(r) => {
                           let ans = ternary!(1, if(l $op r) else 0);
                           Type::Int(ans)
                       },
                       Number::Float(r) => {
                           let ans = ternary!(1, if (l $op r as i64) else 0);
                           Type::Int(ans)
                       }
                   }
                },
                Number::Float(l) => {
                    match $right.into_number() {
                        Number::Float(r) => {
                            let ans = ternary!(1, if(l $op r) else 0);

                            Type::Int(ans)
                        },
                        Number::Int(r) => {
                            let ans = ternary!(1, if(l $op r as f64) else 0);
                            Type::Int(ans)
                        }
                    }
                }
        }
    };
}

#[derive(Debug, Clone)]
pub enum Node {
    Program(Box<Node>),
    Number(String),
    List(Vec<Box<Node>>),
    String(String),
    Node(Box<Node>),
    VarAssign(String, Box<Node>),
    FuncAssign {
        name: String,
        params: Vec<String>,
        body: Box<Node>,
    },
    VarAccess(String),
    Pipe(Box<Node>, Box<Node>),
    If {
        condition: Box<Node>,
        body: Box<Node>,
        else_body: Option<Box<Node>>,
    },
    While {
        condition: Box<Node>,
        code: Box<Node>,
    },
    BinOp(Box<Node>, TT, Box<Node>),
    UnOp(TT, Box<Node>),
    FunctionCall(Box<Node>, Vec<Node>),
    MultiStatement(Vec<Node>),
    StructCreate{name: String, fields: Vec<String> },
    VarDottedAccess{left: Box<Node>, ident: String}
}

macro_rules! str_mul {
    ($str:literal * $num:expr) => {
        $str.to_owned().repeat($num)
    };
}

#[allow(unreachable_patterns)]
impl Node {
    //TODO: impl repr() method

    pub fn repr(&self, indent: usize) -> String {
        match self {
            Node::VarDottedAccess {..} => panic!("VarDottedAccess not implemented for repr()"),
            Node::StructCreate{ .. } => panic!("Struct create repr()"),
            // Node::VarReAssign(name, node) => {
            //     let mut text = "VarReAssign(".to_owned() + "\n";
            //     text += &(str_mul!("\t" * indent + 1) + &name + "\n");
            //     text += &(str_mul!("\t" * indent + 1) + &node.repr(indent + 1) + "\n");
            //     text += &(str_mul!("\t" * indent) + ")");
            //     return text;
            // },
            Node::Node(n) => {
                String::from("Node(") + &n.repr(indent + 1) + ")"
            }
            Node::UnOp(op, right) => {
                let mut text = "UnOp(".to_owned();
                text += &(str_mul!("\t" * indent + 1) + &op.to_string() + "\n");
                text += &(str_mul!("\t" * indent + 1) + &right.repr(indent + 1) + "\n");
                text += &(str_mul!["\t" * indent] + ")");
                return text;
            }
            Node::While{ condition, code } => {
                let mut text = "While(".to_owned();
                text +=&(str_mul!("\t" * indent + 1) + &condition.repr(indent + 1) + "\n");
                text += &(str_mul!("\t" * indent + 1) + &code.repr(indent + 1));
                text += ")";
                return text;
            }
            Node::BinOp(left, op, right) => {
                let mut text = "BinOp(".to_owned() + "\n";
                text += &(str_mul!("\t" * indent + 1) + &left.repr(indent + 1) + "\n");
                text += &(str_mul!("\t" * indent + 1) + &op.to_string() + "\n");
                text += &(str_mul!("\t" * indent + 1) + &right.repr(indent + 1) + "\n");
                text += &(str_mul!("\t" * indent) + ")");
                return text;
            }
            Node::VarAccess(name) => "VarAccess(".to_owned() + &name + ")",
            Node::FuncAssign{name, params, body} => {
                let mut text = "FuncAssign(\n".to_owned();
                text += &(name.to_owned() + "(");
                for param in params {
                    text += &(param.to_owned() + ", ");
                }
                text += &")".to_owned();
                text += &"\n".to_owned();
                text += &str_mul!("\t" * indent + 1);
                text += &body.repr(indent + 1);
                text += &str_mul!("\t" * indent);
                text += ")";
                return text;
            }
            Node::Pipe(left, right) => {
                let mut text = "Pipe(".to_owned() + "\n";
                text += &(str_mul!("\t" * indent + 1) + &left.repr(indent + 1) + "\n");
                text += &(str_mul!("\t" * indent + 1) + "|" + "\n");
                text += &(str_mul!("\t" * indent + 1) + &right.repr(indent + 1) + "\n");
                text += &(str_mul!("\t" * indent) + ")");
                return text;
            }
            Node::VarAssign(name, value) => {
                let mut text = "VarAssign(".to_owned() + "\n";
                text += &(str_mul!("\t" * indent + 1) + &name.to_owned() + "\n");
                text += &(str_mul!("\t" * indent + 1) + &value.repr(indent + 1));
                return text;
            }
            Node::FunctionCall(name, params) => {
                let mut text = "Call(".to_owned() + &name.repr(0) + "(\n";
                for param in params {
                    text += &str_mul!("\t" * indent + 2);
                    text += &param.repr(indent + 2);
                    text += "\n";
                }
                text += &(str_mul!("\t" * indent + 1) + ")\n");
                text += &(str_mul!("\t" * indent) + ")");
                return text;
            }
            Node::MultiStatement(stmts) => {
                let mut text = "MultiStatement(".to_owned() + "\n";
                for stmnt in stmts {
                    text += &(str_mul!("\t" * indent + 1) + &stmnt.repr(indent + 1));
                    text += "\n";
                }
                text += &(str_mul!("\t" * indent) + ")");
                return text;
            }
            Node::String(s) => return "String(".to_owned() + &s + ")",
            Node::Number(x) => {
                return "Number(".to_owned() + &x.to_string() + ")";
            }
            Node::Program(p) => {
                let mut text = "Program(".to_owned() + "\n";
                text += &("\t".to_owned().repeat(indent + 1) + &p.repr(indent + 1) + "\n");
                text += &("\t".to_owned().repeat(indent) + ")");
                return text;
            }
            Node::If {
                condition,
                body,
                else_body,
            } => {
                let mut text = "If(\n".to_owned();
                text += &(str_mul!("\t" * indent + 1) + "Condition(\n");
                text += &condition.repr(indent + 2);
                text += &(str_mul!("\t" * indent + 1) + ")");
                text += &(str_mul!("\t" * indent + 1) + "Body(\n");
                text += &body.repr(indent + 2);
                text += &(str_mul!("\t" * indent + 1) + ")");
                if let Some(ebody) = else_body {
                    text += &(str_mul!("\t" * indent + 1) + "Else(\n");
                    text += &ebody.repr(indent + 2);
                    text += &(str_mul!("\t" * indent + 1) + ")")
                }
                text += &(str_mul!("\t" * indent) + ")");
                return text;
            }
            Node::List(l) => {
                let mut text = "List(\n".to_owned();
                for item in l {
                    text += &str_mul!("\t" * indent + 1);
                    text += &item.repr(indent + 1);
                    text += &",\n".to_owned();
                }
                text += &(str_mul!("\t" * indent) + ")");
                return text;
            }
        }
    }

    pub fn visit(&self, env: &mut SymbolTable) -> Type {
        match self {
            Node::VarDottedAccess{ left, ident } =>{
                let left_type = left.visit(env);
                return left_type.dot_access(ident.to_owned());
            }
            Node::StructCreate{ name, fields } => {
                let r#struct = Type::Struct{name: name.to_owned(), fields: fields.to_owned()};
                env.symbols.insert(name.to_owned(), r#struct.clone());
                return r#struct;
            }
            Node::List(n) => {
                let mut items = vec![];
                for node in n {
                    items.push(Box::new(node.visit(env)));
                }
                return Type::List(items);
            }
            Node::Program(n) => return n.visit(env),
            Node::Node(n) => return Type::Node(n.deref().clone()),
            Node::MultiStatement(n) => {
                if n.len() < 1 {
                    panic!("No nodes");
                }
                let mut res: Option<Type> = None;
                for node in n {
                    res = Some(node.visit(env));
                }
                if let Some(t) = res {
                    return t;
                }
                panic!("No code");
            }
            Node::Number(s) => {
                if s.contains('.') {
                    if let Ok(n) = s.parse::<f64>() {
                        return Type::Float(n);
                    }
                }
                if let Ok(n) = s.parse::<i64>() {
                    return Type::Int(n);
                } else {
                    panic!("Not a number")
                }
            }
            Node::String(s) => Type::String(s.to_string()),
            // Node::VarReAssign(name, node) => {
            //     let val = node.visit(env);
            //     if env.symbols.contains_key(name){
            //         env.symbols.insert(name.to_owned(), val.clone());
            //         val
            //     }
            //     else {
            //         panic!("{} is undefined", name);
            //     }
            // }
            Node::VarAssign(name, node) => {
                let val = node.visit(env);
                env.symbols.insert(name.to_owned(), val.clone());
                val
            }
            Node::VarAccess(name) => env.clone_item(name),
            Node::FuncAssign { name, params, body } => {
                let val = Type::Function {
                    name: name.to_owned(),
                    parameters: params.to_owned(),
                    code: body.deref().to_owned(),
                };
                env.symbols.insert(name.to_owned(), val.clone());
                val
            }
            Node::FunctionCall(name, params) => {
                let func = name.visit(env);
                match &func.access() {
                    Type::Function { .. } | Type::Node(..) | Type::BuiltinFunction { .. } | Type::Struct { .. } => {
                        let mut real_params = vec![];
                        for item in params {
                            let val = item.visit(env);
                            real_params.push(val);
                        }
                        let rv = func.run(real_params, env);
                        return rv;
                    }
                    t => panic!("Cannot call {:?}", t),
                }
            }
            Node::While { condition, code } => {
                let mut res: Type = Type::Int(0);
                while let Number::Int(x) = condition.visit(env).into_number() {
                    if x == 0 {
                        break;
                    }
                    res = code.visit(env)
                }
                res
            }
            Node::Pipe(left, right) => {
                let l = left.visit(env);
                env.symbols.insert("PIPE".to_string(), l);
                right.visit(env)
            }
            Node::If {
                condition,
                body,
                else_body,
            } => {
                let cond_val = condition.visit(env).into_number();

                if let Number::Int(x) = cond_val {
                    if x != 0 {
                        body.visit(env)
                    } else {
                        if let Some(c) = else_body {
                            c.visit(env)
                        } else {
                            return Type::Int(0);
                        }
                    }
                } else {
                    return Type::Int(0);
                }
            }
            Node::BinOp(left, op, right) => {
                let mut l = left.visit(env);
                let r = right.visit(env);
                let ans = match op {
                    TT::MUL => l.mul(&r),
                    TT::DIV => l.div(&r),
                    TT::MINUS => l.sub(&r),
                    TT::PLUS => l.add(&r),
                    TT::GT => compare_numbertype!(l > r),
                    TT::LT => compare_numbertype!(l < r),
                    TT::LE => compare_numbertype!(l <= r),
                    TT::GE => compare_numbertype!(l >= r),
                    TT::EQEQ => compare_numbertype!(l == r),
                    TT::EQ => {
                        match left.deref() {
                            Node::VarDottedAccess{ident, ..} => {
                                let mut left_copy = *(left.clone());
                                while let Node::VarDottedAccess{left: l, ..} = left_copy {
                                    left_copy = *l;
                                }
                                if let Node::VarAccess(name) = left_copy {
                                    let mut s = env.symbols.remove(&name).unwrap();
                                    let val = s.setattr(ident, r);
                                    env.symbols.insert(name, s);
                                    val
                                }
                                else {
                                    panic!("Weird expression");
                                }
                            }
                            Node::VarAccess(ident) => {
                                env.symbols.insert(ident.to_owned(), r.clone());
                                r
                            }
                            _ => panic!("Cannot assign to {:?}", left),
                        }
                    }
                    _ => todo!("[BinOpNode] {:?} Not yet implemented", op),
                };
                ans
            }
            Node::UnOp(op, right) => {
                let r = right.visit(env);
                match op {
                    TT::MINUS => r.mul(&Type::Int(-1)),
                    TT::PLUS => r,
                    _ => panic!("{:?} not a valid unary op", op)
                }
            }
            _ => panic!("{:?} not impl", self),
        }
    }
}
