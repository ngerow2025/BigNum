use big_num::{parse, BigNum};

fn main() {
    let a = parse("100", big_num::Base::Decimal);
    let b = BigNum::from(7u32);
    println!("a = {}", a);
    println!("b = {}", b);
    println!("a + b = {}", a.clone() + b.clone());
    println!("a - b = {}", a.clone() - b.clone());
    println!("a * b = {}", a.clone() * b.clone());
    println!("a / b = {}", a / b);
}
