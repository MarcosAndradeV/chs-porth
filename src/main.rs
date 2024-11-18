#![allow(unused)]
use std::{env::args, fs, path::Path};

use mylexer::{Lexer, TokenKind};
use myparser::Parser;

mod mylexer;
mod myparser;

fn main() -> Result<(), ()> {
    let mut args = args();
    let program = args.next().expect("Program Name");
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "com" => {
                if let Some(file_path) = args.next() {
                    let _ = com(file_path);
                } else {
                    eprintln!("ERROR: No file provioded");
                    usage(&program);
                }
                    break;
            }
            "lex" => {
                if let Some(file_path) = args.next() {
                    let _ = lex(file_path);
                } else {
                    eprintln!("ERROR: No file provioded");
                    usage(&program);
                    break;
                }
            }
            "help" => {
                usage(&program);
                break;
            }
            _ => todo!(),
        }
    }
    Ok(())
}

fn lex(file_path: String) -> Result<(), ()> {
    let source = fs::read_to_string(Path::new(&file_path))
        .map_err(|err| eprintln!("ERROR: Cannot open {} {}", file_path, err))?;
    let mut lex = Lexer::new(file_path, source);
    loop {
        let token = lex.next_token();
        println!("{} {}", token.loc, token);
        if token.kind == TokenKind::EOF {
            break;
        }
    }
    Ok(())
}

fn com(file_path: String) -> Result<(), ()> {
    let source = fs::read_to_string(Path::new(&file_path))
        .map_err(|err| eprintln!("ERROR: Cannot open {} {}", file_path, err))?;
    let ops = Parser::new(Lexer::new(file_path, source)).parse()?;
    println!("{:#?}", ops);
    Ok(())
}

fn usage(program: &String) {
    println!("USAGE: {} <COMMAND>", program);
    println!("COMMANDS: ");
    println!("      com <file> -- Compile program ");
    println!("      lex <file> -- Lexer program");
    println!("      help       -- Print this help message");
}
