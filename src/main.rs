#![warn(clippy::all)]

mod parse;
mod token;

use crate::{parse::*, token::*};
use std::{env, fs, process};

fn main() {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let args = env::args().collect::<Vec<_>>();
    let [_, path, ..] = args.as_slice() else {
        println!("usage: {} <file>", args[0]);
        process::exit(64);
    };
    let src = fs::read_to_string(path).unwrap();
    let ast = parse(tokens(&src));
    println!("{ast}")
}
