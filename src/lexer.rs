use crate::{tokens::{self, TT, Keyword}, util::String_vec, util::str};

macro_rules! adv_or_break {
    ($self:ident) => {
        if !$self.advance() {
            break;
        }
    };
}

#[derive(Clone)]
pub struct Lexer {
    text: String,
    tokens: Vec<tokens::TT>,
    cur_idx: usize,
}

impl Lexer {
    pub fn new(text: String) -> Lexer {
        return Lexer {
            text,
            tokens: vec![],
            cur_idx: 0,
        };
    }

    pub fn get_tokens(&self) -> Vec<tokens::TT> {
        return self.tokens.clone();
    }

    pub fn parse(&mut self) -> Vec<tokens::TT> {
        loop {
            let cur_char = self.get_cur_char();
            let tok = match cur_char {
                '+' => TT::PLUS,
                '-' => TT::MINUS,
                '*' => TT::MUL,
                '/' => TT::DIV,
                '=' | '>' | '<' => self.build_comp(cur_char),
                '(' => TT::LPAREN,
                ')' => TT::RPAREN,
                ';' => TT::SEMI,
                ',' => TT::COMMA,
                '|' => TT::PIPE,
                '[' => TT::LBRACKET,
                ']' => TT::RBRACKET,
                '.' => TT::DOT,
                '0'..='9' => self.build_number(cur_char),
                ' ' | '\t' | '\n' => {
                    adv_or_break!(self);
                    continue;
                }
                '"' => self.build_string(),
                'A'..='Z' | '_' | 'a'..='z' => {
                    let str = self.build_ident_string(cur_char);
                    if let Some(kwd) = Keyword::is_keyword(&str){
                        TT::KEYWORD(kwd)
                    }
                    else {
                        TT::IDENT(str)
                    }
                }
                _ => {
                    panic!("[Lexer] {} Invalid char", cur_char);
                }
            };
            self.tokens.push(tok);
            adv_or_break!(self);
        }
        return self.get_tokens();
    }

    fn build_comp(&mut self, cur_char: char) -> TT {
        let mut comp = cur_char.to_string();

        if self.advance() {
            let ch = self.get_cur_char();
            if ch != '=' {
                self.back();
                match comp.as_str() {
                    "=" => crate::tokens::TT::EQ,
                    ">" => crate::tokens::TT::GT,
                    "<" => crate::tokens::TT::LT,
                    _ => todo!("{} as comp token", comp)
                }
            } else {
                comp += &ch.to_string();
                match comp.as_str() {
                    "==" => TT::EQEQ,
                    ">=" => TT::GE,
                    "<=" => TT::LE,
                    _ => todo!("{} as comp token", comp)
                }
            }
        } else {
            self.back();
            match comp.as_str() {
                "=" => crate::tokens::TT::EQ,
                ">" => crate::tokens::TT::GT,
                "<" => crate::tokens::TT::LT,
                _ => todo!("{} as comp token", comp)
            }
        }
    }

    fn build_ident_string(&mut self, mut cur_char: char) -> String {
        let mut ident = String::from(cur_char);

        while self.advance() {
            cur_char = self.get_cur_char();
            if !match cur_char {
                'A'..='Z' | '_' | 'a'..='z' => true,
                _ => false,
            } {
                break;
            }

            ident += cur_char.to_string().as_str();
        }
        self.back();
        return ident;
    }

    fn build_string(&mut self) -> TT {
        let mut string = String::new();
        let mut cur_char;
        let mut escape = false;
        while self.advance() {
            cur_char = self.get_cur_char();
            if cur_char == '\\' {
                escape = true;
                continue;
            }
            if escape {
                escape = false;
                match cur_char {
                    'n' => cur_char = '\n',
                    't' => cur_char = '\t',
                    _ => {}
                }
            } else if cur_char == '"' {
                break;
            }
            string += cur_char.to_string().as_str();
        }
        return TT::STRING(string);
    }

    fn build_number(&mut self, mut cur_char: char) -> TT {
        let mut number = String::from(cur_char);
        let mut is_float = false;
        while self.advance() {
            cur_char = self.get_cur_char();
            if cur_char == '.' {
                if !is_float {
                    is_float = true;
                } else {
                    break;
                }
            } else if cur_char < '0' || cur_char > '9' {
                break;
            }
            number += cur_char.to_string().as_str();
        }
        //will advance too far if we dont go back
        self.back();
        return TT::NUMBER(number);
    }

    fn get_cur_char(&self) -> char {
        return self.text.as_bytes()[self.cur_idx] as char;
    }

    fn advance(&mut self) -> bool {
        self.cur_idx += 1;
        return self.cur_idx < self.text.len();
    }

    fn back(&mut self) {
        self.cur_idx -= 1;
    }
}
