use regex::Regex;
use std::{ops::Deref, sync::LazyLock};
use unicode_ident::{is_xid_continue, is_xid_start};

pub struct Lexer<'a> {
    src: &'a str,
    peek: Option<Token<'a>>,
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.peek.is_some() {
            return self.peek.take();
        }

        loop {
            self.src = self.src.trim_start();
            if !self.src.starts_with('#') {
                break;
            }
            self.src = self.src.trim_start_matches(|c| c != '\n');
        }

        let token = token(&mut self.src)?;
        self.src = &self.src[token.len()..];
        Some(token)
    }
}
impl<'a> Lexer<'a> {
    pub fn new(src: &str) -> Lexer {
        Lexer { src, peek: None }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if self.peek.is_some() {
            return self.peek.as_ref();
        }

        self.peek = self.next();
        self.peek.as_ref()
    }
}

#[derive(Debug, Clone)]
pub enum Token<'a> {
    Atom(&'a str, &'static str),
    Op(&'a str),
}
impl<'a> Deref for Token<'a> {
    type Target = &'a str;

    fn deref(&self) -> &Self::Target {
        match self {
            Token::Atom(t, _) => t,
            Token::Op(t) => t,
        }
    }
}

fn token<'a>(src: &mut &'a str) -> Option<Token<'a>> {
    let token = match src.chars().next()? {
        c if c.is_numeric() => Token::Atom(number(src)?, "number"),
        c if is_punctuation(c) => Token::Op(operator(src)?), // todo: acceptable punctuation tokens
        c if is_xid_start(c) || c == '_' => match scan(src, is_xid_continue) {
            s @ "true" | s @ "false" => Token::Atom(s, "bool"),
            s @ "unit" => Token::Atom(s, "unit"),
            s if is_keyword(s) => Token::Op(s),
            s => Token::Atom(s, "id"),
        },
        '"' => Token::Atom(text(src)?, "text"),
        c => {
            panic!("bad character: {c:?}");
        }
    };

    Some(token)
}
fn scan(src: &str, pat: fn(char) -> bool) -> &str {
    &src[..src.find(|c| !pat(c)).unwrap()]
}
fn is_punctuation(c: char) -> bool {
    "+-*/%^<>=,.:!?()[]{}".contains(c)
}
fn is_keyword(s: &str) -> bool {
    const KEYWORDS: &[&str] = &[
        "var", "and", "or", "not", "xor", "to", "do", "end", "if", "then", "else", "match", "with",
        "loop", "break", "function", "type", "record", "trait",
    ];
    KEYWORDS.contains(&s)
}
fn operator(s: &str) -> Option<&str> {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"(>=|<=)|==|[+\-*/%^<>=,.:!?()\[\]{}]"#).unwrap());
    RE.find(s).map(|m| m.as_str())
}
fn number(s: &str) -> Option<&str> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^\d+(\.\d+)?"#).unwrap());
    RE.find(s).map(|m| m.as_str())
}
fn text(s: &str) -> Option<&str> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^"([^"\\]|\\[\s\S])*""#).unwrap());
    RE.find(s).map(|m| m.as_str())
}
