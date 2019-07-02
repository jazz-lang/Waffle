use std::fmt;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TokenKind {
    At,
    String(String),
    CString(String),
    LitChar(char),
    LitInt(String, IntBase, IntSuffix),
    LitFloat(String, FloatSuffix),
    Identifier(String),
    BangIdent(String),
    End,

    LQuote,
    RQuote,

    // Keywords
    Fun,
    While,
    If,
    Else,
    Loop,
    Break,
    Continue,
    Return,
    True,
    False,
    Null,
    Pub,
    Static,
    Inline,
    Import,
    Extern,
    Enum,
    Alias,
    Struct,
    Const,
    SizeOf,
    Underscore,
    Defer,
    Var,
    // Operators
    Dollar,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Not,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Semicolon,
    Dot,
    Colon,
    Sep, // ::
    Arrow,
    Tilde,
    BitOr,
    BitAnd,
    Caret,
    And,
    Or,
    Internal,
    For,

    Eq,
    EqEq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    EqEqEq,
    NeEqEq,
    Is,
    As,
    DotDotDot,
    GtGt,
    GtGtGt,
    LtLt,
}

impl TokenKind {
    pub fn name(&self) -> &str {
        match *self {
            TokenKind::Var => "var",
            TokenKind::CString(_) => "c-string",
            TokenKind::String(_) => "string",
            TokenKind::LitInt(_, _, suffix) => match suffix {
                IntSuffix::Byte => "byte number",
                IntSuffix::Int => "int number",
                IntSuffix::Long => "long number",
                IntSuffix::UByte => "unsigned byte number",
                IntSuffix::UInt => "unsigned int number",
                IntSuffix::ULong => "unsigned long number",
            },
            TokenKind::DotDotDot => "...",

            TokenKind::LitChar(_) => "char",

            TokenKind::LitFloat(_, suffix) => match suffix {
                FloatSuffix::Float => "float number",
                FloatSuffix::Double => "double number",
            },
            TokenKind::Import => "import",
            TokenKind::BangIdent(_) => "identifier!",
            TokenKind::Dollar => "$",
            TokenKind::Identifier(_) => "identifier",
            TokenKind::End => "<<EOF>>",
            TokenKind::LQuote => "<",
            TokenKind::RQuote => ">",
            TokenKind::Fun => "func",
            TokenKind::For => "for",
            TokenKind::While => "while",
            TokenKind::If => "if",
            TokenKind::Else => "else",
            TokenKind::Loop => "loop",

            TokenKind::Break => "break",
            TokenKind::Continue => "continue",
            TokenKind::Return => "return",
            TokenKind::True => "true",
            TokenKind::False => "false",
            TokenKind::Null => "null",
            TokenKind::Pub => "pub",
            TokenKind::Static => "static",
            TokenKind::Inline => "inline",
            TokenKind::Extern => "extern",
            TokenKind::Enum => "enum",
            TokenKind::Alias => "alias",
            TokenKind::Struct => "struct",
            TokenKind::Const => "const",
            TokenKind::SizeOf => "sizeof",
            TokenKind::Underscore => "_",
            TokenKind::Defer => "defer",

            // Operators
            TokenKind::At => "@",
            TokenKind::Add => "+",
            TokenKind::Sub => "-",
            TokenKind::Mul => "*",
            TokenKind::Div => "/",
            TokenKind::Mod => "%",
            TokenKind::Not => "!",
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::LBracket => "[",
            TokenKind::RBracket => "]",
            TokenKind::LBrace => "{",
            TokenKind::RBrace => "}",
            TokenKind::Comma => ",",
            TokenKind::Semicolon => ";",
            TokenKind::Dot => ".",
            TokenKind::Colon => ":",
            TokenKind::Sep => "::",
            TokenKind::Arrow => "=>",
            TokenKind::Tilde => "~",
            TokenKind::BitOr => "|",
            TokenKind::BitAnd => "&",
            TokenKind::Caret => "^",
            TokenKind::And => "&&",
            TokenKind::Or => "||",
            TokenKind::Internal => "internal",

            TokenKind::Eq => "=",
            TokenKind::EqEq => "==",
            TokenKind::Ne => "!=",
            TokenKind::Lt => "<",
            TokenKind::Le => "<=",
            TokenKind::Gt => ">",
            TokenKind::Ge => ">=",

            TokenKind::GtGt => ">>",
            TokenKind::GtGtGt => ">>>",
            TokenKind::LtLt => "<<",

            TokenKind::EqEqEq => "===",
            TokenKind::NeEqEq => "!==",
            TokenKind::Is => "is",
            TokenKind::As => "as",
        }
    }
}

use crate::ast::Location;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub position: Location,
}

impl Token {
    pub fn new(tok: TokenKind, pos: Location) -> Token {
        Token {
            kind: tok,
            position: pos,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.kind == TokenKind::End
    }

    pub fn is(&self, kind: TokenKind) -> bool {
        self.kind == kind
    }

    pub fn name(&self) -> String {
        match self.kind {
            TokenKind::LitInt(ref val, _, suffix) => {
                let suffix = match suffix {
                    IntSuffix::Byte => "B",
                    IntSuffix::Int => "",
                    IntSuffix::Long => "L",
                    IntSuffix::UByte => "UB",
                    IntSuffix::UInt => "UI",
                    IntSuffix::ULong => "UL",
                };

                format!("{}{}", val, suffix)
            }

            TokenKind::String(ref val) => format!("\"{}\"", &val),
            TokenKind::Identifier(ref val) => val.clone(),

            _ => self.kind.name().into(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.name())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum IntBase {
    Bin,
    Dec,
    Hex,
}

impl IntBase {
    pub fn num(self) -> u32 {
        match self {
            IntBase::Bin => 2,
            IntBase::Dec => 10,
            IntBase::Hex => 16,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd)]
pub enum IntSuffix {
    Int,
    Long,
    Byte,
    ULong,
    UInt,
    UByte,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd)]
pub enum FloatSuffix {
    Float,
    Double,
}

use crate::err::*;
use crate::reader::Reader;
use std::collections::HashMap;

pub struct Lexer {
    pub reader: Reader,
    keywords: HashMap<&'static str, TokenKind>,
}

impl std::str::FromStr for Lexer {
    type Err = ();
    fn from_str(s: &str) -> Result<Lexer, ()> {
        Ok(Lexer::from_str_(s))
    }
}

impl Lexer {
    pub fn from_str_(code: &str) -> Lexer {
        let reader = Reader::from_string(code);
        Lexer::new(reader)
    }

    pub fn new(reader: Reader) -> Lexer {
        let keywords = keywords_in_map();

        Lexer { reader, keywords }
    }

    pub fn path(&self) -> &str {
        self.reader.path()
    }

    pub fn read_token(&mut self) -> Result<Token, MsgWithPos> {
        loop {
            self.skip_white();

            let pos = self.reader.pos();
            let ch = self.cur();

            if ch.is_none() {
                return Ok(Token::new(TokenKind::End, pos));
            }

            if is_digit(ch) {
                return self.read_number();
            } else if self.is_comment_start() {
                self.read_comment()?;
            } else if ch == Some('c') {
                if is_quote(self.next()) {
                    self.read_char();
                    let string = if let TokenKind::String(s) = &self.read_string().unwrap().kind {
                        Token::new(TokenKind::CString(s.clone()), pos)
                    } else {
                        unreachable!()
                    };

                    return Ok(string);
                } else {
                    return self.read_identifier();
                }
            } else if self.is_multi_comment_start() {
                self.read_multi_comment()?;
            } else if is_identifier_start(ch) {
                return self.read_identifier();
            } else if ch == Some('$') {
                self.read_char();
                return Ok(self.build_token(TokenKind::Dollar));
            } else if is_quote(ch) {
                return self.read_string();
            } else if is_char_quote(ch) {
                return self.read_char_literal();
            } else if is_operator(ch) {
                return self.read_operator();
            } else if is_macro_call_start(ch) {
                unimplemented!()
            } else {
                let ch = ch.unwrap();

                return Err(MsgWithPos::new(
                    self.reader.path().to_string(),
                    self.reader.src.clone(),
                    pos,
                    Msg::UnknownChar(ch),
                ));
            }
        }
    }

    fn skip_white(&mut self) {
        while is_whitespace(self.cur()) {
            self.read_char();
        }
    }

    fn read_comment(&mut self) -> Result<(), MsgWithPos> {
        while self.cur().is_some() && !is_newline(self.cur()) {
            self.read_char();
        }

        Ok(())
    }

    fn read_multi_comment(&mut self) -> Result<(), MsgWithPos> {
        let pos = self.reader.pos();

        self.read_char();
        self.read_char();

        while self.cur().is_some() && !self.is_multi_comment_end() {
            self.read_char();
        }

        if self.cur().is_none() {
            return Err(MsgWithPos::new(
                self.reader.path().to_string(),
                self.reader.src.clone(),
                pos,
                Msg::UnclosedComment,
            ));
        }

        self.read_char();
        self.read_char();

        Ok(())
    }

    fn read_identifier(&mut self) -> Result<Token, MsgWithPos> {
        let pos = self.reader.pos();
        let mut value = String::new();

        while is_identifier(self.cur()) {
            let ch = self.cur().unwrap();
            self.read_char();
            value.push(ch);
        }

        let lookup = self.keywords.get(&value[..]).cloned();
        let ttype;

        if let Some(tok_type) = lookup {
            ttype = tok_type;
        } else if value == "_" {
            ttype = TokenKind::Underscore;
        } else if value == "!" {
            ttype = TokenKind::BangIdent(value);
        } else {
            ttype = TokenKind::Identifier(value);
        }

        Ok(Token::new(ttype, pos))
    }

    fn read_char_literal(&mut self) -> Result<Token, MsgWithPos> {
        let pos = self.reader.pos();

        self.read_char();
        let ch = self.read_escaped_char(pos.clone(), Msg::UnclosedChar)?;

        if is_char_quote(self.cur()) {
            self.read_char();

            let ttype = TokenKind::LitChar(ch);
            Ok(Token::new(ttype, pos.clone()))
        } else {
            Err(MsgWithPos::new(
                self.reader.path().to_string(),
                self.reader.src.clone(),
                pos,
                Msg::UnclosedChar,
            ))
        }
    }

    fn read_escaped_char(&mut self, pos: Location, unclosed: Msg) -> Result<char, MsgWithPos> {
        if let Some(ch) = self.cur() {
            self.read_char();

            if ch == '\\' {
                let ch = if let Some(ch) = self.cur() {
                    ch
                } else {
                    return Err(MsgWithPos::new(
                        self.reader.path().to_string(),
                        self.reader.src.clone(),
                        pos,
                        unclosed,
                    ));
                };

                self.read_char();

                match ch {
                    '\\' => Ok('\\'),
                    'n' => Ok('\n'),
                    't' => Ok('\t'),
                    'r' => Ok('\r'),
                    '\"' => Ok('\"'),
                    '\'' => Ok('\''),
                    '0' => Ok('\0'),

                    'e' => unimplemented!(),
                    'v' => unimplemented!(),
                    'x' => unimplemented!(),
                    'u' => unimplemented!(),

                    _ => {
                        let msg = Msg::InvalidEscapeSequence(ch);
                        Err(MsgWithPos::new(
                            self.reader.path().to_string(),
                            self.reader.src.clone(),
                            pos,
                            msg,
                        ))
                    }
                }
            } else {
                Ok(ch)
            }
        } else {
            Err(MsgWithPos::new(
                self.reader.path().to_string(),
                self.reader.src.clone(),
                pos,
                unclosed,
            ))
        }
    }

    fn read_string(&mut self) -> Result<Token, MsgWithPos> {
        let pos = self.reader.pos();
        let mut value = String::new();

        self.read_char();

        while self.cur().is_some() && !is_quote(self.cur()) {
            let ch = self.read_escaped_char(pos.clone(), Msg::UnclosedString)?;
            value.push(ch);
        }

        if is_quote(self.cur()) {
            self.read_char();

            let ttype = TokenKind::String(value);
            Ok(Token::new(ttype, pos))
        } else {
            Err(MsgWithPos::new(
                self.reader.path().to_string(),
                self.reader.src.clone(),
                pos,
                Msg::UnclosedString,
            ))
        }
    }

    fn read_operator(&mut self) -> Result<Token, MsgWithPos> {
        let mut tok = self.build_token(TokenKind::End);
        let ch = self.cur().unwrap();
        self.read_char();

        let nch = self.cur().unwrap_or('x');
        let nnch = self.next().unwrap_or('x');

        tok.kind = match ch {
            '+' => TokenKind::Add,
            '-' => {
                if nch == '>' {
                    self.read_char();
                    TokenKind::Arrow
                } else {
                    TokenKind::Sub
                }
            }

            '*' => TokenKind::Mul,
            '/' => TokenKind::Div,
            '%' => TokenKind::Mod,

            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,

            '|' => {
                if nch == '|' {
                    self.read_char();
                    TokenKind::Or
                } else {
                    TokenKind::BitOr
                }
            }

            '&' => {
                if nch == '&' {
                    self.read_char();
                    TokenKind::And
                } else {
                    TokenKind::BitAnd
                }
            }
            '$' => TokenKind::Dollar,

            '^' => TokenKind::Caret,
            '~' => TokenKind::Tilde,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            ':' => {
                if nch == ':' {
                    self.read_char();
                    TokenKind::Sep
                } else {
                    TokenKind::Colon
                }
            }
            '.' => {
                if nch == '.' {
                    self.read_char();
                    if self.cur() == Some('.') {
                        self.read_char();
                        TokenKind::DotDotDot
                    } else {
                        // TODO: ..=
                        unimplemented!()
                    }
                } else {
                    TokenKind::Dot
                }
            }
            '=' => {
                if nch == '=' {
                    self.read_char();

                    if nnch == '=' {
                        self.read_char();
                        TokenKind::EqEqEq
                    } else {
                        TokenKind::EqEq
                    }
                } else {
                    TokenKind::Eq
                }
            }

            '<' => match nch {
                '=' => {
                    self.read_char();
                    TokenKind::Le
                }

                '<' => {
                    self.read_char();
                    TokenKind::LtLt
                }

                _ => TokenKind::Lt,
            },

            '>' => match nch {
                '=' => {
                    self.read_char();
                    TokenKind::Ge
                }

                '>' => {
                    self.read_char();

                    if nnch == '>' {
                        self.read_char();
                        TokenKind::GtGtGt
                    } else {
                        TokenKind::GtGt
                    }
                }

                _ => TokenKind::Gt,
            },

            '!' => {
                if nch == '=' {
                    self.read_char();

                    if nnch == '=' {
                        self.read_char();
                        TokenKind::NeEqEq
                    } else {
                        TokenKind::Ne
                    }
                } else {
                    TokenKind::Not
                }
            }

            _ => {
                return Err(MsgWithPos::new(
                    self.reader.path().to_string(),
                    self.reader.src.clone(),
                    tok.position,
                    Msg::UnknownChar(ch),
                ));
            }
        };

        Ok(tok)
    }

    fn read_number(&mut self) -> Result<Token, MsgWithPos> {
        let pos = self.reader.pos();
        let mut value = String::new();

        let base = if self.cur() == Some('0') {
            let next = self.next();

            match next {
                Some('x') => {
                    self.read_char();
                    self.read_char();

                    IntBase::Hex
                }

                Some('b') => {
                    self.read_char();
                    self.read_char();

                    IntBase::Bin
                }

                _ => IntBase::Dec,
            }
        } else {
            IntBase::Dec
        };

        self.read_digits(&mut value, base);

        if base == IntBase::Dec && self.cur() == Some('.') && is_digit(self.next()) {
            self.read_char();
            value.push('.');

            self.read_digits(&mut value, IntBase::Dec);

            if self.cur() == Some('e') || self.cur() == Some('E') {
                value.push(self.cur().unwrap());
                self.read_char();

                if self.cur() == Some('+') || self.cur() == Some('-') {
                    value.push(self.cur().unwrap());
                    self.read_char();
                }

                self.read_digits(&mut value, IntBase::Dec);
            }

            let suffix = match self.cur() {
                Some('D') => {
                    self.read_char();
                    FloatSuffix::Double
                }

                Some('F') => {
                    self.read_char();
                    FloatSuffix::Float
                }

                _ => FloatSuffix::Double,
            };

            let ttype = TokenKind::LitFloat(value, suffix);
            return Ok(Token::new(ttype, pos));
        }

        let suffix = match self.cur() {
            Some('L') => {
                self.read_char();
                IntSuffix::Long
            }

            Some('Y') => {
                self.read_char();
                IntSuffix::Byte
            }

            Some('D') if base == IntBase::Dec => {
                self.read_char();

                let ttype = TokenKind::LitFloat(value, FloatSuffix::Double);
                return Ok(Token::new(ttype, pos));
            }

            Some('F') if base == IntBase::Dec => {
                self.read_char();

                let ttype = TokenKind::LitFloat(value, FloatSuffix::Float);
                return Ok(Token::new(ttype, pos));
            }
            Some('U') if base == IntBase::Dec => {
                self.read_char();
                let suffix = match self.cur() {
                    Some('B') | Some('Y') => IntSuffix::UByte,
                    Some('I') => IntSuffix::Int,
                    Some('L') => IntSuffix::ULong,
                    _ => IntSuffix::UInt,
                };
                self.read_char();
                suffix
            }

            _ => IntSuffix::Int,
        };

        let ttype = TokenKind::LitInt(value, base, suffix);
        Ok(Token::new(ttype, pos))
    }

    fn read_digits(&mut self, buffer: &mut String, base: IntBase) {
        while is_digit_or_underscore(self.cur(), base) {
            let ch = self.cur().unwrap();
            self.read_char();
            buffer.push(ch);
        }
    }

    fn read_char(&mut self) {
        self.reader.advance();
    }

    fn cur(&self) -> Option<char> {
        self.reader.cur()
    }

    fn next(&self) -> Option<char> {
        self.reader.next()
    }

    fn build_token(&self, kind: TokenKind) -> Token {
        Token::new(kind, self.reader.pos())
    }

    fn is_comment_start(&self) -> bool {
        self.cur() == Some('/') && self.next() == Some('/')
    }

    fn is_multi_comment_start(&self) -> bool {
        self.cur() == Some('/') && self.next() == Some('*')
    }

    fn is_multi_comment_end(&self) -> bool {
        self.cur() == Some('*') && self.next() == Some('/')
    }
}

fn is_digit(ch: Option<char>) -> bool {
    ch.map(|ch| ch.is_digit(10)).unwrap_or(false)
}

fn is_digit_or_underscore(ch: Option<char>, base: IntBase) -> bool {
    ch.map(|ch| ch.is_digit(base.num()) || ch == '_')
        .unwrap_or(false)
}

fn is_whitespace(ch: Option<char>) -> bool {
    ch.map(|ch| ch.is_whitespace()).unwrap_or(false)
}

fn is_newline(ch: Option<char>) -> bool {
    ch == Some('\n')
}

fn is_quote(ch: Option<char>) -> bool {
    ch == Some('\"')
}

fn is_char_quote(ch: Option<char>) -> bool {
    ch == Some('\'')
}

fn is_operator(ch: Option<char>) -> bool {
    ch.map(|ch| "^+-*/%&|,=!~;:.()[]{}<>".contains(ch))
        .unwrap_or(false)
}

fn is_identifier_start(ch: Option<char>) -> bool {
    match ch {
        Some(ch) => (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '_',
        _ => false,
    }
}

fn is_identifier(ch: Option<char>) -> bool {
    is_identifier_start(ch) || is_digit(ch)
}

fn is_macro_call_start(ch: Option<char>) -> bool {
    ch == Some('@')
}

fn keywords_in_map() -> HashMap<&'static str, TokenKind> {
    let mut keywords = HashMap::new();

    keywords.insert("func", TokenKind::Fun);
    keywords.insert("for", TokenKind::For);
    keywords.insert("var", TokenKind::Var);
    keywords.insert("while", TokenKind::While);
    keywords.insert("if", TokenKind::If);
    keywords.insert("else", TokenKind::Else);
    keywords.insert("inline", TokenKind::Inline);
    keywords.insert("extern", TokenKind::Extern);
    keywords.insert("import", TokenKind::Import);
    keywords.insert("loop", TokenKind::Loop);
    keywords.insert("break", TokenKind::Break);
    keywords.insert("continue", TokenKind::Continue);
    keywords.insert("return", TokenKind::Return);
    keywords.insert("true", TokenKind::True);
    keywords.insert("false", TokenKind::False);
    keywords.insert("null", TokenKind::Null);
    keywords.insert("enum", TokenKind::Enum);
    keywords.insert("alias", TokenKind::Alias);
    keywords.insert("struct", TokenKind::Struct);
    keywords.insert("sizeof", TokenKind::SizeOf);
    keywords.insert("defer", TokenKind::Defer);
    keywords.insert("as", TokenKind::As);
    keywords.insert("internal", TokenKind::Internal);

    keywords.insert("pub", TokenKind::Pub);
    keywords.insert("static", TokenKind::Static);

    keywords.insert("const", TokenKind::Const);

    keywords
}
