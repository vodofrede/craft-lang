use crate::token::*;
use std::fmt;

pub fn parse(src: &str) -> Expr {
    let mut lexer = Lexer::new(src);
    do_block(&mut lexer).unwrap()
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

fn prod<I, F>(lexer: &mut Lexer, name: &str, productions: I) -> Expr
where
    I: IntoIterator<Item = F>,
    F: Fn(&mut Lexer) -> Option<Expr>,
{
    Expr::Cons(
        name.to_string(),
        productions.into_iter().filter_map(|p| p(lexer)).collect(),
    )
}

// productions
fn skip(lexer: &mut Lexer) -> Option<Expr> {
    lexer.next();
    None
}

fn block<F>(lexer: &mut Lexer, name: &str, f: F, end: &str) -> Option<Expr>
where
    F: Fn(&mut Lexer) -> Option<Expr>,
{
    let mut block = vec![];
    while let Some(token) = lexer.peek() {
        if token.eq(end) {
            break;
        }
        let Some(e) = f(lexer) else { break };
        block.push(e);
    }
    Some(Expr::Cons(name.to_string(), block))
}
fn do_block(lexer: &mut Lexer) -> Option<Expr> {
    block(lexer, "do", expr, "end")
}
fn if_block(lexer: &mut Lexer) -> Option<Expr> {
    block(lexer, "do", expr, "else")
}

fn expr(lexer: &mut Lexer) -> Option<Expr> {
    expr_bp(lexer, 0)
}
fn expr_bp(lexer: &mut Lexer, min_power: usize) -> Option<Expr> {
    // parse primary
    let mut left = match lexer.next()? {
        Token::Atom(text, kind) => Expr::Atom(text.to_string(), kind),
        Token::Op("(") => eat(expr(lexer)?, || lexer.next()),
        Token::Op("do") => prod(lexer, "do", [do_block, skip]),
        Token::Op("if") => prod(lexer, "if", [expr, skip, if_block, skip, do_block, skip]),
        Token::Op("match") => prod(lexer, "match", [expr, skip, match_arms, skip]),
        Token::Op(op) if unary_power(op).is_some() => {
            Expr::Cons(op.to_string(), vec![expr_bp(lexer, unary_power(op)?)?])
        }
        t => {
            eprintln!("bad token: {t:?}");
            return None;
        }
    };

    // parse infix operators
    loop {
        let Some(Token::Op(op)) = lexer.peek() else {
            break;
        };
        let Some((left_power, right_power)) = infix_power(op) else {
            break;
        };
        if left_power < min_power {
            break;
        }
        let Some(Token::Op(op)) = lexer.next() else {
            break;
        };
        let right = expr_bp(lexer, right_power)?;
        left = match op {
            "(" => eat(Expr::Cons(left.to_string(), vec![right]), || lexer.next()),
            _ => Expr::Cons(op.to_string(), vec![left, right]),
        }
    }

    Some(left)
}

fn match_arms(lexer: &mut Lexer) -> Option<Expr> {
    block(lexer, "with", match_arm, "end")
}
fn match_arm(lexer: &mut Lexer) -> Option<Expr> {
    Some(prod(lexer, "case", [skip, expr, skip, expr]))
}

// binding power
fn infix_power(op: &str) -> Option<(usize, usize)> {
    let power = match op {
        "=" => (2, 1),
        "to" => (3, 4),
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
    let power = match op {
        "-" | "not" => 19,
        "var" => 23,
        _ => return None,
    };

    Some(power)
}

// util
fn eat<'a>(expr: Expr, mut f: impl FnMut() -> Option<Token<'a>>) -> Expr {
    f();
    expr
}
