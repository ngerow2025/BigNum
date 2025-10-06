use big_num::{encode, parse, Base};

fn main() {
    let n = parse("255.5", Base::Decimal);
    let hex = encode(n.clone(), Base::Hexadecimal);
    let b64 = encode(n, Base::Base64);
    println!("hex = {}, base64 = {}", hex, b64);
}
