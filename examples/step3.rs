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

fn main()
{
    let mut args: Vec<String> = env::args().collect();

    let num = args.len();
    if num != 2
    {
        eprintln!("引数に異常あり：{}", num);
    }

    let AsmFilename = args.pop().unwrap();
    let AsmFileReslt = fs::File::open(AsmFilename.clone());
    match AsmFileReslt
    {
        Ok(v) =>{
            fs::remove_file(AsmFilename.clone()).unwrap();
        }
        Err(e) =>{
        }
    }

    let mut AsmFile = fs::File::create(AsmFilename.clone()).unwrap();
    let mut AsmFile = OpenOptions::new().read(true).write(true).open(AsmFilename).unwrap();

    AsmFile.write_all(b".intel_syntax noprefix\n").unwrap();
    AsmFile.write_all(b".global main\n").unwrap();
    AsmFile.write_all(b"main:\n").unwrap();
    AsmFile.write_all(b" mov rax, ").unwrap();

    let Token = tokenize(args.pop().unwrap());

    let mut n = expect_number();
    AsmFile.write_all(n).unwrap();

    loop
    {
        if at_eof() == true
        {
            break;
        }

        if consume('+') == true
        {
            AsmFile.write_all(b" add rax, ").unwrap();
            let mut n = expect_number();
            AsmFile.write_all(n).unwrap();
            continue;        
        }

        expect('-');
        AsmFile.write_all(b" sub rax, ").unwrap();
        let mut n = expect_number();
        AsmFile.write_all(n).unwrap();
    }

    AsmFile.write_all(b" ret\n").unwrap();
}

