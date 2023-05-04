#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    Var,
    While,
    End,
    Struct,
    If,
    Then,
    Fi,
    Code,
    Edoc,
    Rav,
    Do,
    Else
}

impl Keyword{
    pub fn as_str(&self) -> &str {
        match self {
            Keyword::End => "end",
            Keyword::Struct => "struct",
            Keyword::Fi => "fi",
            Keyword::Do => "do",
            Keyword::If => "if",
            Keyword::While => "while",
            Keyword::Var => "var",
            Keyword::Rav => "rav",
            Keyword::Then => "then",
            Keyword::Code => "code",
            Keyword::Edoc => "edoc",
            Keyword::Else => "else"
        }
    }

    pub fn is_keyword(word: &str) -> Option<Keyword> {
        match word {
            "struct" => Some(Keyword::Struct),
            "end" => Some(Keyword::End),
            "fi" => Some(Keyword::Fi),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "rav" => Some(Keyword::Rav),
            "var" => Some(Keyword::Var),
            "then" => Some(Keyword::Then),
            "code" => Some(Keyword::Code),
            "edoc" => Some(Keyword::Edoc),
            "do" => Some(Keyword::Do),
            "while" => Some(Keyword::While),
            _ => None
        }
    }
}

#[derive(Clone, Debug)]
pub enum TT{
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,
    SEMI,
    EQ,
    GT,
    GE,
    LT,
    LE,
    EQEQ,
    COMMA,
    KEYWORD(Keyword),
    IDENT(String),
    STRING(String),
    NUMBER(String),
    PIPE,
    DOT
}

impl TT{
    pub fn to_string(&self) -> String {
         match self {
            TT::DOT => return ".".to_owned(),
            TT::PLUS => return "+".to_string(),
            TT::MINUS => return "-".to_string(),
            TT::MUL => return "*".to_string(),
            TT::DIV => return "/".to_string(),
            TT::LPAREN => return "(".to_string(),
            TT::RPAREN => return ")".to_string(),
            TT::LBRACKET => return "[".to_string(),
            TT::RBRACKET => return "]".to_string(),
            TT::SEMI => return ";".to_string(),
            TT::EQ => return "=".to_string(),
            TT::GT => return ">".to_string(),
            TT::LT => return "<".to_string(),
            TT::GE => return ">=".to_string(),
            TT::LE => return "<=".to_string(),
            TT::EQEQ => return "==".to_string(),
            TT::COMMA => return ",".to_string(),
            TT::KEYWORD(kwd) => return String::from("Keyword(") + kwd.as_str() + ")",
            TT::IDENT(ident) => return String::from("Ident(") + &ident + ")",
            TT::STRING(s) => return String::from("String(") + s + ")",
            TT::NUMBER(s) => return String::from("Number(") + s + ")",
            TT::PIPE => return "|".to_string()
        };
    }
}
