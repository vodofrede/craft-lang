use regex::Regex;
use std::{
    iter::{self, Peekable},
    sync::LazyLock,
};
use unicode_ident::{is_xid_continue, is_xid_start};

pub fn tokens(mut src: &str) -> Peekable<impl Iterator<Item = Token> + '_> {
    iter::from_fn(move || {
        loop {
            src = src.trim_start();
            if !src.starts_with('#') {
                break;
            }
            src = src.trim_start_matches(|c| c != '\n');
        }
        let token = token(&mut src)?;
        eprintln!("token: {:?}", token.text());
        src = &src[token.text().len()..];
        Some(token)
    })
    .peekable()
}
fn token(src: &mut &str) -> Option<Token> {
    let token = match src.chars().next()? {
        c if c.is_numeric() => Token::Atom(number(src)?.to_string(), "number"),
        c if is_punctuation(c) => Token::Op(scan(src, is_punctuation).to_string()), // todo: acceptable punctuation tokens
        c if is_xid_start(c) || c == '_' => match scan(src, is_xid_continue) {
            s @ "true" | s @ "false" => Token::Atom(s.to_string(), "bool"),
            s @ "unit" => Token::Atom(s.to_string(), "unit"),
            s if is_keyword(s) => Token::Op(s.to_string()),
            s => Token::Atom(s.to_string(), "id"),
        },
        '"' => Token::Atom(text(src)?.to_string(), "text"),
        c => {
            panic!("bad character: {c:?}");
        }
    };

    Some(token)
}

#[derive(Debug, Clone)]
pub enum Token {
    Atom(String, &'static str),
    Op(String),
}
impl Token {
    pub fn text(&self) -> &str {
        match self {
            Token::Atom(t, _) => t,
            Token::Op(t) => t,
        }
    }
}

fn scan(src: &str, pat: fn(char) -> bool) -> &str {
    &src[..src.find(|c| !pat(c)).unwrap()]
}
fn is_punctuation(c: char) -> bool {
    "+-*/%^<>=,.:!?()[]".contains(c)
}
fn is_keyword(s: &str) -> bool {
    const KEYWORDS: &[&str] = &[
        "var", "and", "or", "not", "xor", "to", "do", "end", "if", "then", "else", "match", "with",
        "loop", "break", "type", "record", "trait",
    ];
    KEYWORDS.contains(&s)
}
fn number(s: &str) -> Option<&str> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^\d+(\.\d+)?"#).unwrap());
    RE.find(s).map(|m| m.as_str())
}
fn text(s: &str) -> Option<&str> {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^"([^"\\]|\\[\s\S])*""#).unwrap());
    RE.find(s).map(|m| m.as_str())
}
