use std::io::Write;

use big_num::{codec, BigNum};

mod ast;

use ast::AstNode;

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn convert() {
    //ask for the number to convert
    let input_string = input("Enter a number: ");

    //ask the user for the base
    let input_base_string = input("What base is this number in? (2, 8, 10, 16, 64) ");

    //ask the user for the base to convert to
    let output_base_string = input("What base do you want to convert to? (2, 8, 10, 16, 64) ");

    let input_base = input_base_string.parse::<codec::Base>().unwrap();
    let output_base = output_base_string.parse::<codec::Base>().unwrap();

    let input = codec::parse(input_string, input_base);
    let output = codec::encode(input, output_base);

    println!("result: {}", output);
}

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub calculator);

fn parse(input: &str) -> AstNode {
    let ast = calculator::ExprParser::new().parse(input).unwrap();
    ast
}

fn eval(input: &AstNode) -> BigNum {
    match input {
        AstNode::Literal(x) => x.clone(),
        AstNode::Add(x, y) => eval(x) + eval(y),
        AstNode::Sub(x, y) => eval(x) - eval(y),
        AstNode::Mul(x, y) => eval(x) * eval(y),
        AstNode::Div(x, y) => eval(x) / eval(y),
        AstNode::Pow(x, y) => eval(x).pow(eval(y)),
        AstNode::Mod(x, y) => eval(x) % eval(y),
    }
}

fn evaluate() {
    let input = input("Enter an expression: ");
    let ast = parse(&input);
    let res = eval(&ast);
    println!("result: {}", res);
}

fn main() {
    println!("Welcome to the BigNum calculator!");
    println!("options:");
    println!("(1) Convert a number from one base to another");
    println!("(2) Evaluate a mathematical expression");

    loop {
        let option = input("What do you want to do? (1, 2) ");
        match option.as_str() {
            "1" => convert(),
            "2" => evaluate(),
            _ => println!("Invalid option"),
        }
    }

    // let mut output: BigNum;
    // {
    //     let _div = ScopeTimer::new("div", TimeFormat::Milliseconds, None, false);

    //     output = BigNum::from(1) / BigNum::from(3);
    // }

    // {
    //     let _print = ScopeTimer::new("print", TimeFormat::Milliseconds, None, false);
    //     println!("{}", output);
    // }
}
