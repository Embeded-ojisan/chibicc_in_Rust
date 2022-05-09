/*
    cargo run step1 (formula) (file).s
    という実行の仕方を期待する
*/
use std::env;
use std::fs;
use std::io::Write;
use std::io::Read;
use std::io::BufReader;
use std::fs::OpenOptions;


fn main()
{
    let mut args: Vec<String> = env::args().collect();

    let num = args.len();
    if num != 3
    {
        eprintln!("引数に異常あり：{}", num);
    }

    let AsmFilename = args.pop().unwrap();
//    let mut AsmFile = fs::File::create(AsmFilename).unwrap();
    let mut AsmFile = OpenOptions::new().read(true).write(true).open(AsmFilename).unwrap();

    AsmFile.write_all(b".intel_syntax noprefix\n").unwrap();
    AsmFile.write_all(b".global main\n").unwrap();
    AsmFile.write_all(b"main:\n").unwrap();
    AsmFile.write_all(b" mov rax, ").unwrap();

//    let mut formula = args.pop().unwrap();
//    AsmFile.write_all(formula.bytes().unwrap()).unwrap();

/*
    let mut FormulaFile = fs::File::create("tmp_formula_step2").unwrap();
    let mut FormulaFile = OpenOptions::new().read(true).write(true).open("tmp_formula_step2").unwrap();
    FormulaFile.write_all(args.pop().unwrap().as_bytes());
    
    let mut FormulaFile = fs::File::create("tmp_formula_step2").unwrap();
    let mut FormulaFile = OpenOptions::new().read(true).write(true).open("tmp_formula_step2").unwrap();
    let mut buf = [0; 256];
    println!("{}\n", FormulaFile.read(&mut buf).expect("something went wrong reading the file"));
*/
//    let mut bytes = args.pop().unwrap().as_bytes();
//    AsmFile.write_all(&args.pop().unwrap().as_bytes().iter().next().unwrap());
    println!("{}", args.pop().unwrap().as_bytes().iter().next().unwrap());

//    println!("{:?}", buf);
}