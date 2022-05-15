/*
    cargo run step1 (formula) (file).s
    という実行の仕方を期待する
*/
use std::env;
use std::fs;
use std::fs::OpenOptions;

use std::io::Write;
use std::io::Read;
use std::io::BufReader;
use std::io::Seek;

fn main()
{
    let mut args: Vec<String> = env::args().collect();

    let num = args.len();
    if num != 3
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

    let mut buf = args.pop().unwrap().into_bytes();
    let len = buf.len();

    let mut temp = String::new(); 
    for cc in &buf[..len]
    {
        if *cc == b'+'
        {
            let len = temp.len();
            temp.insert_str(len, "\n add rax, ");
            continue;
        }

        if *cc == b'-'
        {
            let len = temp.len();
            temp.insert_str(len, "\n sub rax, ");
            continue;
        }

        let len = temp.len();
        temp.insert_str(len, &(cc.to_ascii_lowercase()-48).to_string());
    }
    let len = temp.len();
    temp.insert_str(len, "\n ret"); 

    AsmFile.write_all(temp.as_bytes()).unwrap();
}