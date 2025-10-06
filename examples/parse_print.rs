use big_num::{parse, Base, BigNum};

fn main() {
    let a: BigNum = parse("123.45", Base::Decimal);
    println!("Parsed decimal: {}", a);
}
