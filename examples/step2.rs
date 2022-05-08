/*
    cargo run step1 (formula) (file).s
    という実行の仕方を期待する
*/
use std::env;
use std::fs;
use std::io::Write;
use std::io::Read;
use std::io::BufReader;

fn main()
{
    let mut args: Vec<String> = env::args().collect();

    let num = args.len();
    if num != 3
    {
        eprintln!("引数に異常あり：{}", num);
    }

    let AsmFilename = args.pop();
    let mut AsmFile = fs::File::create(AsmFilename.unwrap()).unwrap();

    AsmFile.write_all(b".intel_syntax noprefix\n").unwrap();
    AsmFile.write_all(b".global main\n").unwrap();
    AsmFile.write_all(b"main:\n").unwrap();
    AsmFile.write_all(b" mov rax, ").unwrap();

//    let mut formula = args.pop().unwrap();
//    AsmFile.write_all(formula.bytes().unwrap()).unwrap();

    let mut FormulaFile = fs::File::create("tmp_formula_step2").unwrap();
    FormulaFile.write_all(args.pop().unwrap().as_bytes());
    let mut buf = BufReader::new(FormulaFile).bytes();

    println!("{:x}", buf);
}