use std::collections::LinkedList;
use std::fmt::*;

use crate::TokenKind::*;

#[derive(PartialEq)]
enum TokenKind {
    TK_RESERVED,
    TK_NUM,
    TK_EOF
}

struct Token {
    kind: TokenKind,
    val: usize,
    str: String,
}

fn error(args: Arguments)
{
    eprintln!("{}/n", args);
}

fn consume(token: &mut LinkedList<Token>, op: u8) -> bool
{
    if token.len() > 1
    {
        let str = token.pop_front().str.as_bytes();
        if token.kind != TK_RESERVED || str[0] != op
        {
            return false;
        }
    }
    return true;
}

pub fn main()
{

}

