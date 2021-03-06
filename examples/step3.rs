use std::collections::LinkedList;
use std::fmt::*;

use std::fs;
use std::fs::File;
use std::fs::OpenOptions;

use std::env;

use std::io::Write;
use std::io::Read;
use std::io::BufReader;
use std::io::Seek;

use crate::TokenKind::*;

#[derive(PartialEq)]
enum TokenKind {
    TK_RESERVED,
    TK_NUM,
    TK_EOF,
}

pub struct Token {
    kind:   TokenKind,
    val:    usize,
    str:    String,
    next:   Option<Box<Token>>,
}

struct TokenList {
    head: Option<Box<Token>>,
}

impl TokenList{
    pub fn new() -> TokenList {
        Self {
            head: None,
        }
    }

    pub fn new_token(
        &mut self,
        kind: TokenKind,
        str: &mut String
    )
    {
        let mut new_token = 
            Token{
                kind:   kind,
                val:    0,
                str:    *str,
                next:   None,
            };

        match self.head {
            None => self.head = Some(Box::new(new_token)),
            Some(ref mut head) => {
                let mut p = head;
                loop {
                    match p.next {
                        None => {
                            p.next = Some(Box::new(new_token));
                            break;
                        },
                        Some(ref mut next) => p = next,
                    }
                }
            }
        }
    }

    pub fn get(
        &self
        ,index: isize
    ) -> Option<Box<Token>>
    {
        match self.head
        {
            None => return None,
            Some(ref top) => {
                let mut p = top;
                let mut i = 0;

                loop
                {
                    if i == index
                    {
                        return Some(*p);
                    }

                    match p.next
                    {
                        None => return None,
                        Some(ref link) => p = link,
                    }
                    i += 1;
                }
            }
        }
    }

    pub fn len(
        &self
    ) -> usize
    {
        let mut count = 1;
        match self.head
        {
            None => 0,
            Some(ref top) => {
                let mut p = top;

                loop
                {
                    match p.next
                    {
                        None => break,
                        Some(ref link) => {
                            count += 1;
                            continue;
                        }
                    }
                }

                return count;
            }
        }
    }

    pub fn at_eof(
        &mut self
    ) -> Option<TokenKind>
    {
        match self.len()
        {
            0 => None,
            _ => {
                Some(self.head.unwrap().kind)
            }
        } 
    }

    pub fn consume(&mut self, op: u8) -> bool
    {
        if self.len() > 1
        {
            let str = self.get(0).unwrap().str.as_bytes();
            if self.at_eof().unwrap() != TK_RESERVED 
                || str[0] != op
            {
                return false;
            }
        }
        return true;
    }

    pub fn tokenize(
        &mut self
        ,op: &mut Vec<u8>
    )
    {
        let mut index = 0;
        if self.len() > 1
        {
            let str = 
                self
                    .get(0)
                    .unwrap()
                    .str
                    .into_bytes();
            loop
            {
                if char::from_u32(op[index] as u32).unwrap() == '\0'
                {
                    break;
                }
                else if char::from_u32(op[index] as u32).unwrap() == ' '
                {
                    index += 1;
                    continue;
                }

                if char::from_u32(op[index] as u32).unwrap() == '+' 
                || char::from_u32(op[index] as u32).unwrap() == '-'
                {
                    index += 1;
                    self.new_token(
                        TK_RESERVED
                        ,&mut String::from_utf8(
                            op[index..].to_vec()
                        ).unwrap()
                    );
                    continue;
                }
                
                if char::from_u32(op[index] as u32)
                    .unwrap()
                    .is_digit(10)
                       == true
                {
                   self.new_token(
                       TK_NUM
                       ,&mut String::from_utf8(
                           op[index..].to_vec()
                        ).unwrap()
                   );
                   self
                    .get(self.len().try_into().unwrap())
                    .unwrap()
                    .val
                    = op[index] as usize;
                   continue;
               }

            }
        }

        self.new_token(
            TK_NUM
            ,&mut String::from_utf8(
                op[index..].to_vec()
            ).unwrap()
        );
        return;
    }

    pub fn expect(
        &mut self
        ,op: &mut Vec<u8>
    ) -> Option<usize>
    {
        match self.head {
            None => None,
            Some(ref mut head) => {
                let mut p = head;
                let token = p.kind;
                let inOp = self.get(0).unwrap().str.as_bytes();
                match p.next {
                    None => None,
                    Some(ref mut next) =>{
                        if token != TK_RESERVED
                            || inOp != op 
                        {
                            println!("{:?}?????????????????????", op);
                        }
                        
                        p = next;
                        Some(1)
                    },
                }
            }
        }
    }

    pub fn expect_number(
        &mut self
    )
    {
        ;
    }
}

/*
fn error(
    args: Arguments
)
{
    eprintln!("{}/n", args);
}
*/


fn main()
{
    let mut args: Vec<String> = env::args().collect();

    let num = args.len();
    if num != 2
    {
        eprintln!("????????????????????????{}", num);
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

/*
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
*/
}

