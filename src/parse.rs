use crate::token::*;
use std::fmt;

pub fn parse(src: &str) -> Expr {
    let mut lexer = Lexer::new(src);

    let mut exprs = vec![];
    while lexer.peek().is_some() {
        let Some(expr) = expr(&mut lexer) else {
            break;
        };
        exprs.push(expr);
    }

    Expr::Cons("do".to_string(), exprs)
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
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

fn prod<I, P>(lexer: &mut Lexer, name: &str, productions: I) -> Expr
where
    I: IntoIterator<Item = P>,
    P: Fn(&mut Lexer) -> Option<Expr>,
{
    Expr::Cons(
        name.to_string(),
        productions.into_iter().filter_map(|p| p(lexer)).collect(),
    )
}
fn eat<'a>(expr: Expr, mut f: impl FnMut() -> Option<Token<'a>>) -> Expr {
    f();
    expr
}

fn expr(lexer: &mut Lexer) -> Option<Expr> {
    expr_bp(lexer, 0)
}
fn expr_bp(lexer: &mut Lexer, min_power: usize) -> Option<Expr> {
    // parse primary
    let mut left = match lexer.next()? {
        Token::Atom(text, kind) => Expr::Atom(text.to_string(), kind),
        Token::Op(op) if unary_power(op).is_some() => {
            Expr::Cons(op.to_string(), vec![expr_bp(lexer, unary_power(op)?)?])
        }
        Token::Op("(") => eat(seq(lexer)?, || lexer.next()),
        Token::Op("[") => eat(list(lexer)?, || lexer.next()),
        Token::Op("{") => eat(table(lexer)?, || lexer.next()),
        Token::Op("do") => eat(do_block(lexer)?, || lexer.next()),
        Token::Op("if") => prod(lexer, "if", [expr, skip, then_block, skip, do_block, skip]),
        Token::Op("match") => prod(lexer, "match", [expr, skip, match_arms, skip]),
        Token::Op("loop") => prod(lexer, "loop", [do_block, skip]),
        Token::Op("break") => Expr::Cons("break".to_string(), vec![]),
        Token::Op("function") => prod(lexer, "function", [expr, do_block, skip]),
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
        left = match op {
            "(" => prod(lexer, &left.to_string(), [params, skip]),
            _ => Expr::Cons(op.to_string(), vec![left, expr_bp(lexer, right_power)?]),
        }
    }

    Some(left)
}

// productions

// ignore the next token
fn skip(lexer: &mut Lexer) -> Option<Expr> {
    lexer.next();
    None
}

// parse multiple expressions until end is found
fn block<P>(lexer: &mut Lexer, name: &str, p: P, end: &str) -> Option<Expr>
where
    P: Fn(&mut Lexer) -> Option<Expr>,
{
    let mut block = vec![];
    loop {
        let token = lexer.peek()?;
        if token.eq(end) {
            break;
        }
        let expr = p(lexer)?;
        block.push(expr);
    }
    Some(Expr::Cons(name.to_string(), block))
}
fn do_block(lexer: &mut Lexer) -> Option<Expr> {
    block(lexer, "do", expr, "end")
}
fn then_block(lexer: &mut Lexer) -> Option<Expr> {
    block(lexer, "do", expr, "else")
}

// parse match arms
fn match_arms(lexer: &mut Lexer) -> Option<Expr> {
    block(lexer, "with", match_arm, "end")
}
fn match_arm(lexer: &mut Lexer) -> Option<Expr> {
    Some(prod(lexer, "case", [skip, expr, skip, expr]))
}

// parse list of expressions separated by commas
fn comma(lexer: &mut Lexer, name: &str) -> Option<Expr> {
    let mut exprs = vec![expr(lexer)?];
    while let Some(token) = lexer.peek() {
        if token.ne(",") {
            break;
        }
        lexer.next(); // consume comma
        let expr = expr(lexer)?;
        exprs.push(expr);
    }
    Some(Expr::Cons(name.to_string(), exprs))
}
fn seq(lexer: &mut Lexer) -> Option<Expr> {
    comma(lexer, "seq").map(|seq| match seq {
        Expr::Cons(_, exprs) if exprs.len() == 1 => exprs[0].clone(),
        _ => seq,
    })
}
fn list(lexer: &mut Lexer) -> Option<Expr> {
    comma(lexer, "list")
}
fn table(lexer: &mut Lexer) -> Option<Expr> {
    comma(lexer, "table")
}
fn params(lexer: &mut Lexer) -> Option<Expr> {
    comma(lexer, "params")
}

// binding power
fn infix_power(op: &str) -> Option<(usize, usize)> {
    let power = match op {
        "=" => (4, 3),
        ":" => (5, 6),
        "to" => (7, 8),
        "or" => (9, 10),
        "xor" => (11, 12),
        "and" => (13, 14),
        "==" => (15, 16),
        ">" | "<" | ">=" | "<=" => (17, 18),
        "+" | "-" => (19, 20),
        "*" | "/" => (21, 22),
        "(" => (23, 24),
        "." => (28, 29),
        _ => return None,
    };

    Some(power)
}
fn unary_power(op: &str) -> Option<usize> {
    let power = match op {
        "-" | "not" => 21,
        "var" => 25,
        _ => return None,
    };

    Some(power)
}
