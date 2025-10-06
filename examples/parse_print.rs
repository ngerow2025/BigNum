use big_num::{parse, BigNum, Base};

fn main() {
    let a: BigNum = parse("123.45", Base::Decimal);
    println!("Parsed decimal: {}", a);
}
