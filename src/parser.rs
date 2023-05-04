use crate::nodes::Node;
use crate::tokens::Keyword;
// use crate::nodes::{
// self, BinOpNode, FuncAssignNode, FunctionCallNode, IfNode, MultiStatementNode, ProgramNode,
// VarAssignNode, WhileNode,
// };
use crate::util::{str, String_vec};
use crate::{nodes, tokens::TT};

pub struct Parser {
    tokens: Vec<TT>,
    cur_idx: usize,
    endblock_keyewords: Vec<String>,
}

impl Parser {
    pub fn new(tokens: Vec<TT>) -> Parser {
        return Parser {
            tokens,
            cur_idx: 0,
            endblock_keyewords: String_vec!["done", "fi", "rav"],
        };
    }

    pub fn parse(&mut self) -> nodes::Node {
        return self.program();
    }

    fn get_cur_tok(&self) -> Option<TT> {
        if self.cur_idx < self.tokens.len() {
            return Some(self.tokens[self.cur_idx].clone());
        }
        return None;
    }

    fn literal(&mut self) -> nodes::Node {
        match self.get_cur_tok() {
            Some(TT::KEYWORD(Keyword::Code)) => {
                self.advance();
                let res = Node::Node(Box::new(self.multi_statement()));
                if let Some(TT::KEYWORD(Keyword::Edoc)) = self.get_cur_tok() {
                    self.advance();
                    return res;
                }
                panic!("Expected 'edoc' to end node literal");
            }
            Some(TT::LBRACKET) => {
                self.advance();
                let mut items: Vec<Box<nodes::Node>> = vec![Box::new(self.statement())];
                while let Some(TT::COMMA) = self.get_cur_tok() {
                    self.advance();
                    items.push(Box::new(self.statement()));
                }
                match self.get_cur_tok() {
                    Some(TT::RBRACKET) => {
                        self.advance();
                        nodes::Node::List(items)
                    }
                    _ => panic!("List must end with ]"),
                }
            }
            Some(TT::NUMBER(x)) => {
                self.advance();
                return nodes::Node::Number(x);
            }
            Some(TT::STRING(s)) => {
                self.advance();
                return nodes::Node::String(s);
            }
            Some(TT::IDENT(name)) => {
                self.advance();
                return nodes::Node::VarAccess(name);
            }
            _ => panic!("Curr toke is none"),
        }
    }

    fn atom(&mut self) -> nodes::Node {
        match self.get_cur_tok() {
            Some(TT::IDENT(i)) => {
                self.advance();
                let ident = nodes::Node::VarAccess(i);
                match self.get_cur_tok() {
                    Some(TT::EQ) => {
                        self.advance();
                        return nodes::Node::BinOp(Box::new(ident), TT::EQ, Box::new(self.statement()));
                    }
                    _ => ident,
                }
            }
            _ => self.literal(),
        }
    }

    fn factor(&mut self) -> nodes::Node {
        let mut left;
        if let Some(TT::LPAREN) = self.get_cur_tok() {
            self.advance();
            left = self.multi_statement();
            if let Some(TT::RPAREN) = self.get_cur_tok() {
                self.advance();
            }
            else{
                panic!("Expected ')'")
            }
        }
        else {
            left = self.atom();
        }

        while let Some(TT::DOT) = self.get_cur_tok(){
            self.advance();
            if let Some(TT::IDENT(right)) = self.get_cur_tok(){
                left = nodes::Node::VarDottedAccess{left: Box::new(left), ident: right};
                self.advance();
            }
            else {
                break;
            }
        }

        return left;
    }

    fn unop(&mut self) -> nodes::Node {
        let curtok = self.get_cur_tok();
        let left = match curtok {
            Some(TT::PLUS) => {
                self.advance();
                nodes::Node::UnOp(TT::PLUS, Box::new(self.factor()))
            }
            Some(TT::MINUS) => {
                self.advance();
                nodes::Node::UnOp(TT::MINUS, Box::new(self.factor()))
            }
            _ => self.factor(),
        };
        if let Some(TT::LPAREN) = self.get_cur_tok() {
            let mut nodes: Vec<nodes::Node> = vec![];
            self.advance();
            while match self.get_cur_tok() {
                Some(TT::RPAREN) => false,
                Some(TT::COMMA) => {
                    self.advance();
                    true
                }
                _ => true,
            } {
                nodes.push(self.statement());
            }
            self.advance();
            return nodes::Node::FunctionCall(Box::new(left), nodes);
        } else {
            left
        }
    }

    fn term(&mut self) -> nodes::Node {
        let mut left = self.unop();
        loop {
            if let Some(tok) = self.get_cur_tok() {
                match tok {
                    TT::DIV | TT::MUL => {
                        self.advance();
                        left = nodes::Node::BinOp(Box::new(left), tok, Box::new(self.unop()));
                    }
                    _ => {
                        break;
                    }
                }
            } else {
                break;
            }
        }
        return left;
    }

    fn arith(&mut self) -> nodes::Node {
        let mut left = self.term();
        loop {
            if let Some(tok) = self.get_cur_tok() {
                match tok {
                    TT::PLUS | TT::MINUS => {
                        self.advance();
                        left = nodes::Node::BinOp(Box::new(left), tok, Box::new(self.term()));
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        return left;
    }

    fn expr(&mut self) -> nodes::Node {
        return self.arith();
    }

    fn pipe(&mut self) -> nodes::Node {
        let mut left = self.expr();
        while let Some(TT::PIPE) = self.get_cur_tok() {
            self.advance();
            left = nodes::Node::Pipe(Box::new(left), Box::new(self.expr()));
        }
        return left;
    }

    fn var_assign(&mut self) -> nodes::Node {
        let ident = self.get_cur_tok();
        match ident {
            Some(TT::IDENT(name)) => {
                self.advance();
                match self.get_cur_tok() {
                    Some(TT::EQ) => {
                        self.advance();
                        return nodes::Node::VarAssign(name.clone(), Box::new(self.statement()));
                    }
                    Some(TT::LPAREN) => {
                        self.advance();
                        let mut params: Vec<String> = vec![];
                        while let Some(TT::IDENT(x)) = self.get_cur_tok() {
                            params.push(x);
                            self.advance();
                            match self.get_cur_tok() {
                                Some(TT::COMMA) => {
                                    self.advance();
                                    continue;
                                }
                                Some(TT::RPAREN) => break,
                                _ => panic!("Expected ')' or ','"),
                            }
                        }
                        self.advance();
                        match self.get_cur_tok() {
                            Some(TT::EQ) => {
                                self.advance();
                            }
                            _ => panic!("Expected '='"),
                        }
                        let stmnt = self.multi_statement();
                        match self.get_cur_tok() {
                            Some(TT::KEYWORD(Keyword::Rav)) => {
                                self.advance();
                            }
                            _ => panic!("Expected 'rav'"),
                        }
                        return nodes::Node::FuncAssign {
                            name,
                            params,
                            body: Box::new(stmnt),
                        };
                    }
                    _ => panic!("NOt done"),
                };
            }
            Some(..) | None => panic!("Expected identifer"),
        }
    }

    fn if_statement(&mut self) -> nodes::Node {
        let condition = self.statement();

        if let Some(TT::KEYWORD(Keyword::Then)) = self.get_cur_tok() {
            self.advance();
            let program = self.multi_statement();
            let mut else_code: Option<Box<nodes::Node>> = None;
            while let Some(TT::KEYWORD(kwd)) = self.get_cur_tok() {
                match kwd {
                    Keyword::Else => {
                        self.advance();
                        else_code = Some(Box::new(self.multi_statement()))
                    }
                    Keyword::Fi => {
                        self.advance();
                        return nodes::Node::If {
                            condition: Box::new(condition),
                            body: Box::new(program),
                            else_body: else_code,
                        };
                    }
                    _ => break,
                }
            }
            panic!("Expected 'then' after condition")
        }
        panic!("Expected 'then' after condition")
    }

    fn while_loop(&mut self) -> nodes::Node {
        let condition = self.statement();

        if let Some(TT::KEYWORD(Keyword::Do)) = self.get_cur_tok() {
            self.advance();
            let program = self.multi_statement();
            if let Some(TT::KEYWORD(Keyword::End)) = self.get_cur_tok() {
                self.advance();
                return nodes::Node::While {
                    condition: Box::new(condition),
                    code: Box::new(program),
                };
            }
            panic!("Expected 'end' to end while loop")
        }
        panic!("Expected 'do' to start while loop")
    }

    fn comp(&mut self) -> nodes::Node {
        let left = self.pipe();
        let tok = self.get_cur_tok();
        if let Some(t) = tok {
            match t {
                TT::LT | TT::GE | TT::GT | TT::LE | TT::EQEQ => {
                    self.advance();
                    nodes::Node::BinOp(Box::new(left), t, Box::new(self.pipe()))
                }
                _ => left,
            }
        } else {
            return left;
        }
    }

    fn struct_create(&mut self) -> nodes::Node {
        if let Some(TT::IDENT(struct_name)) = self.get_cur_tok() {
            self.advance();
            if let Some(TT::EQ) = self.get_cur_tok() {
                let mut names = vec![];
                self.advance();
                while let Some(TT::IDENT(name)) = self.get_cur_tok() {
                    names.push(name);
                    self.advance();
                    if let Some(TT::COMMA) = self.get_cur_tok() {
                        self.advance();
                        continue;
                    }
                    break;
                }
                if let Some(TT::KEYWORD(Keyword::End)) = self.get_cur_tok() {
                    self.advance();
                    return nodes::Node::StructCreate {
                        name: struct_name,
                        fields: names,
                    };
                }
                panic!("Expected 'end' to end struct");
            } else {
                panic!("Expected '=' after struct name");
            }
        } else {
            panic!("Expected identifier after 'struct'");
        }
    }

    fn statement(&mut self) -> nodes::Node {
        let res: nodes::Node = match self.get_cur_tok() {
            Some(TT::KEYWORD(Keyword::Var)) => {
                self.advance();
                self.var_assign()
            }
            Some(TT::KEYWORD(Keyword::If)) => {
                self.advance();
                self.if_statement()
            }
            Some(TT::KEYWORD(Keyword::While)) => {
                self.advance();
                self.while_loop()
            }
            Some(TT::KEYWORD(Keyword::Struct)) => {
                self.advance();
                self.struct_create()
            }
            Some(_t) => self.comp(),
            None => panic!("Statements must be an expression, var, if, or while"),
        };
        return res;
    }

    fn multi_statement(&mut self) -> nodes::Node {
        let mut nodes: Vec<nodes::Node> = vec![self.statement()];
        while let Some(TT::SEMI) = self.get_cur_tok() {
            self.advance();
            if let None = self.get_cur_tok() {
                break;
            }
            nodes.push(self.statement());
        }
        return nodes::Node::MultiStatement(nodes);
    }

    fn program(&mut self) -> nodes::Node {
        return nodes::Node::Program(Box::new(self.multi_statement()));
    }

    fn advance(&mut self) -> bool {
        self.cur_idx += 1;
        return self.cur_idx < self.tokens.len();
    }
}
