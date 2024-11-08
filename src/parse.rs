use crate::token::*;
use std::{fmt, iter::Peekable};

pub fn parse(mut lexer: Peekable<impl Iterator<Item = Token>>) -> Expr {
    block(&mut lexer, &[])
}

// productions
fn block(lexer: &mut Peekable<impl Iterator<Item = Token>>, end: &[&str]) -> Expr {
    let mut block = vec![];
    loop {
        let Some(token) = lexer.peek() else { break };
        if matches!(token, Token::Op(t) if end.contains(&t.as_ref())) {
            break;
        }
        let Some(e) = expr(lexer, 0) else { break };
        block.push(e);
    }
    Expr::Cons("do".to_string(), block)
}
fn expr(lexer: &mut Peekable<impl Iterator<Item = Token>>, min_power: usize) -> Option<Expr> {
    let mut left = match lexer.next()? {
        Token::Atom(t, k) => Expr::Atom(t, k),
        Token::Op(op) if op == "(" => eat(expr(lexer, 0)?, || lexer.next()),
        Token::Op(op) if op == "do" => eat(block(lexer, &["end"]), || lexer.next()),
        Token::Op(op) if op == "if" => {
            let cond = eat(expr(lexer, 0)?, || lexer.next());
            let body = block(lexer, &["end", "else"]);
            match lexer.next()? {
                Token::Op(t) if t == "else" => {
                    let opt = eat(block(lexer, &["end"]), || lexer.next());
                    Expr::Cons(op, vec![cond, body, opt])
                }
                Token::Op(t) if t == "end" => Expr::Cons(op, vec![cond, body]),
                _ => return None,
            }
        }
        Token::Op(op) if op == "match" => return None,
        Token::Op(op) if unary_power(&op).is_some() => {
            println!("unary op: {op}");
            let power = unary_power(&op)?;
            let right = expr(lexer, power)?;
            Expr::Cons(op, vec![right])
        }
        t => {
            eprintln!("bad token: {t:?}");
            return None;
        }
    };
    loop {
        let Some(Token::Op(op)) = lexer.peek().cloned() else {
            break;
        };
        let Some((left_power, right_power)) = infix_power(&op) else {
            break;
        };
        if left_power < min_power {
            break;
        }

        lexer.next();
        let right = expr(lexer, right_power)?;
        left = match op.as_str() {
            "(" => eat(Expr::Cons(left.to_string(), vec![right]), || lexer.next()),
            _ => Expr::Cons(op.clone(), vec![left, right]),
        }
    }

    Some(left)
}
fn pattern(lexer: &mut Peekable<impl Iterator<Item = Token>>) -> Expr {
    todo!()
}

// binding power
fn infix_power(op: &str) -> Option<(usize, usize)> {
    let power = match op {
        "," => (1, 2),
        "=" => (4, 3),
        "or" => (5, 6),
        "xor" => (7, 8),
        "and" => (9, 10),
        "==" => (11, 12),
        ">" | "<" | ">=" | "<=" => (13, 14),
        "+" | "-" => (15, 16),
        "*" | "/" => (17, 18),
        "(" => (20, 21),
        "." => (24, 25),
        _ => return None,
    };

    Some(power)
}
fn unary_power(op: &str) -> Option<usize> {
    println!("unary op? {op}");
    let power = match op {
        "+" | "-" | "not" => 19,
        "var" => 23,
        _ => return None,
    };

    Some(power)
}

pub enum Expr {
    Atom(String, &'static str),
    Cons(String, Vec<Expr>),
}
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Atom(t, _) => write!(f, "{t}"),
            Expr::Cons(op, vs) => {
                write!(f, "({op}")?;
                vs.iter().try_for_each(|v| write!(f, " {v}"))?;
                f.write_str(")")
            }
        }
    }
}

fn eat(expr: Expr, mut f: impl FnMut() -> Option<Token>) -> Expr {
    f();
    expr
}
fn maybe(lexer: &mut Peekable<impl Iterator<Item = Token>>, opt: &str) {
    if let Some(token) = lexer.peek() {
        if token.text() == opt {
            lexer.next();
        }
    }
}
