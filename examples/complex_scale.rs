use big_num::{BigNum, encode, Base};

fn main() {
    // Build a small fractional value
    let frac = BigNum::from(1u32) / BigNum::from(3u32); // 1/3

    // Compute 2^100 as a BigNum
    let two = BigNum::from(2u32);
    let two_pow_100 = two.pow(BigNum::from(100u32));

    // Multiply fraction by the large integer and encode the result
    let scaled = frac * two_pow_100;
    let out = encode(scaled, Base::Base64);
    println!("(1/3) * 2^100 in base64 = {}", out);
}
