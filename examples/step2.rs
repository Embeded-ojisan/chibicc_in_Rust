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

    let mut FormulaFile = fs::File::create("tmp_formula_step2").unwrap();
    let mut FormulaFile = OpenOptions::new().read(true).write(true).open("tmp_formula_step2").unwrap();
    FormulaFile.write_all(args.pop().unwrap().as_bytes());

    FormulaFile.rewind().unwrap();

    let mut reader = BufReader::new(FormulaFile).bytes();
    AsmFile.write_all(
        &reader
            .next().unwrap().unwrap()
            .to_string()
            .parse::<i32>().unwrap()
            .to_string()
            .as_bytes()
    ).unwrap();

    // クリーンアップ
//    fs::remove_file("tmp_formula_step2").unwrap();
}