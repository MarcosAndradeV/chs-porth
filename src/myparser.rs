#![allow(unused)]

use crate::mylexer::{Lexer, Token, TokenKind};

#[derive(Debug, Clone)]
pub enum Intrinsic {
    Plus,
    Minus,
    Mult,
    Divmod,
    Idivmod,
    Max,
    Eq,
    Gt,
    Lt,
    Ge,
    Le,
    Ne,
    Shr,
    Shl,
    Or,
    And,
    Not,
    Print,
    Dup,
    Swap,
    Drop,
    Over,
    Rot,
    Load8,
    Store8,
    Load16,
    Store16,
    Load32,
    Store32,
    Load64,
    Store64,
    CastPtr,
    CastInt,
    CastBool,
    CastAddr,
    Argc,
    Argv,
    Envp,
    Syscall0,
    Syscall1,
    Syscall2,
    Syscall3,
    Syscall4,
    Syscall5,
    Syscall6,
    Debug,
}

#[derive(Debug, Clone)]
pub enum OpKind {
    PushInt(i64),
    PushBool(bool),
    PushPtr(usize),
    PushAddr(usize),
    PushGlobalMem(usize),
    PushLocalMen(usize),
    PushStr(usize),
    PushCstr(usize),
    Intrinsic(Intrinsic),
    If(usize),
    IfStar(usize),
    Else(usize),
    EndIf,
    EndWhile(usize),
    PrepProc(usize),
    Ret(usize),
    Call(usize),
    Inlined(usize),
    While(usize),
    Do(usize),
    CallLike(usize),
    BindLet(usize),
    BindPeek(usize),
    UnBind(usize),
    PushBind(usize),
}

#[derive(Debug, Clone)]
pub struct Op {
    kind: OpKind,
    token: Token,
}

#[derive(Debug, Clone)]
pub struct Proc {
    name: Token,
    ins: Vec<Token>,
    outs: Vec<Token>,
    body: Vec<Op>,
}

#[derive(Debug, Clone)]
pub enum TopLevel {
    Proc(Proc),
}

pub struct Parser {
    lex: Lexer,
    peeked: Option<Token>,
    tpls: Vec<TopLevel>,
}

impl Parser {
    pub fn new(lex: Lexer) -> Parser {
        Parser {
            tpls: vec![],
            lex,
            peeked: None,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<TopLevel>, ()> {
        loop {
            if let Ok(token) = self.require_valid() {
                match token.kind {
                    TokenKind::Keyword if token.val() == "proc" => self.parse_proc()?,
                    _ => todo!(),
                }
            } else {
                break;
            }
        }
        Ok(self.tpls.clone())
    }

    pub fn parse_proc(&mut self) -> Result<(), ()> {
        let name = self.expect(TokenKind::Identfier)?;
        let mut ins = vec![];
        let mut outs = vec![];
        loop {
            let tok =
                self.expect_many(&[TokenKind::Keyword, TokenKind::Identfier, TokenKind::Punct])?;
            match tok.kind {
                TokenKind::Identfier => ins.push(tok),
                TokenKind::Punct if tok.val() == "--" => {
                    loop {
                        let tok = self.expect_many(&[TokenKind::Keyword, TokenKind::Identfier])?;
                        if tok.kind == TokenKind::Identfier {
                            outs.push(tok);
                        } else if tok.kind == TokenKind::Keyword && tok.val() == "in" {
                            break;
                        }
                    }
                    break;
                }
                TokenKind::Keyword => break,
                _ => unreachable!(),
            }
        }
        let mut body = vec![];
        loop {
            let token = self.require_valid()?;
            match token.kind {
                TokenKind::Integer => {
                    let parse = token.value.parse().map_err(|err| eprintln!("{}", err))?;
                    body.push(Op {
                        kind: OpKind::PushInt(parse),
                        token,
                    });
                }
                TokenKind::Operator => {
                    let kind = match token.val() {
                        "+" => OpKind::Intrinsic(Intrinsic::Plus),
                        _ => {
                            eprintln!(
                                "{} Syntax Error: Unexpected operator {} in proc {}",
                                token.loc, token, name.value
                            );
                            return Err(());
                        }
                    };
                    body.push(Op { kind, token });
                }
                TokenKind::Identfier => {
                    let kind = match token.val() {
                        "print" => OpKind::Intrinsic(Intrinsic::Print),
                        _ => {
                            eprintln!(
                                "{} Syntax Error: Unexpected identifier {} in proc {}",
                                token.loc, token, name.value
                            );
                            return Err(());
                        }
                    };
                    body.push(Op { kind, token });
                }
                TokenKind::Keyword if token.val() == "end" => {
                    break;
                }
                _ => {
                    eprintln!(
                        "{} Syntax Error: Unexpected {} in proc {}",
                        token.loc, token, name.value
                    );
                    return Err(());
                }
            }
        }

        self.tpls.push(TopLevel::Proc(Proc {
            name,
            ins,
            outs,
            body,
        }));
        Ok(())
    }

    fn next(&mut self) -> Token {
        self.peeked.take().unwrap_or_else(|| self.lex.next_token())
    }

    fn peek(&mut self) -> &Token {
        if self.peeked.is_none() {
            self.peeked = Some(self.next());
        }
        self.peeked.as_ref().unwrap()
    }

    fn require_valid(&mut self) -> Result<Token, ()> {
        let token = self.next();

        if token.kind != TokenKind::EOF && token.kind != TokenKind::Invalid {
            return Ok(token);
        }

        eprintln!(
            "{} Syntax Error: Expected a valid token but got {}",
            token.loc, token
        );
        Err(())
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, ()> {
        let token = self.next();

        if token.kind == kind {
            return Ok(token);
        }
        eprintln!(
            "{} Syntax Error: Expected {:?} but got {}",
            token.loc, kind, token
        );
        Err(())
    }

    fn expect_many(&mut self, kind: &[TokenKind]) -> Result<Token, ()> {
        let token = self.next();

        if kind.contains(&token.kind) {
            return Ok(token);
        }
        eprintln!(
            "{} Syntax Error: Expected {:?} but got {}",
            token.loc, kind, token
        );
        Err(())
    }
}
