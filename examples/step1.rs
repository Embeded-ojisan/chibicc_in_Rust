/*
    cargo run step1 (num) (file).s
    という実行の仕方を期待する
*/
use std::env;
use std::fs;
use std::io::Write;

fn main()
{
    let mut args: Vec<String> = env::args().collect();

/*
    let num = args.len();
    if num != 2
    {
        eprintln!("引数に異常あり：{}", num);
    }
*/

    let AsmFilename = args.pop();
    let mut AsmFile = fs::File::create(AsmFilename.unwrap()).unwrap();

    AsmFile.write_all(b".intel_syntax noprefix\n").unwrap();

    AsmFile.write_all(b".global main\n").unwrap();
    
    AsmFile.write_all(b"main:\n").unwrap();
    
    AsmFile.write_all(b" mov rax, ").unwrap();
    AsmFile.write_all(args.pop().unwrap().as_bytes()).unwrap();
    AsmFile.write_all(b"\n").unwrap();
    
    AsmFile.write_all(b" ret\n").unwrap();
}