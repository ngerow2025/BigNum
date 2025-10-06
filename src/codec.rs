use std::{
    error::Error,
    fmt::{self, Display},
    str::FromStr,
};


use crate::big_num::BigNum;

const CHARS: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ+/";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Base {
    Binary = 2,
    Octal = 8,
    Decimal = 10,
    Hexadecimal = 16,
    Base64 = 64,
}

#[derive(Debug)]
pub struct BaseParseError {
    message: String,
}

impl fmt::Display for BaseParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for BaseParseError {}

impl FromStr for Base {
    type Err = BaseParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Base::Binary),
            "8" => Ok(Base::Octal),
            "10" => Ok(Base::Decimal),
            "16" => Ok(Base::Hexadecimal),
            "64" => Ok(Base::Base64),
            _ => Err(BaseParseError {
                message: format!("Invalid base: `{}`", s),
            }),
        }
    }
}

pub fn parse<T: AsRef<str>>(_input: T, base: Base) -> BigNum {
    let mut input = _input.as_ref().to_string();
    let mut result = BigNum::new();
    //check for negative sign
    let negative = input.starts_with('-');
    if negative {
        input.remove(0);
    }
    //check for optional + sign
    if input.starts_with('+') {
        input.remove(0);
    }
    //check for decimal point
    if let Some(decimal) = input.find('.') {
        //get the part before the decimal point
        let before = input[..decimal].to_string();
        //get the part after the decimal point
        let after = input[decimal + 1..].to_string();
        //convert the part before the decimal point
        let before_val = parse(before, base);
        //convert the part after the decimal point
        let after_val = parse(&after, base);
        //divide the part after the decimal point by the base to the power of the number of digits after the decimal point
        let after_val = after_val / BigNum::from(base as u64).pow(BigNum::from(after.len() as u64));
        //add the two parts together
        result = before_val + after_val;
    } else {
        //convert the input to a BigNum
        for c in input.chars() {
            let digit = CHARS.find(c).map(|x| x as u32).expect("Invalid digit");
            assert!(digit < base as u32);
            result = result * BigNum::from(base as u64) + BigNum::from(digit);
        }
    }
    //if the number is negative, negate it
    if negative {
        result = -result;
    }
    result
}

pub fn encode(mut input: BigNum, base: Base) -> String {
    let mut result = String::new();
    let big_base = BigNum::from(base as u64);
    let zero = BigNum::from(0);

    input.compact();

    let target_digits =
        (input.get_precision() as f64 * 64f64 / ((base as u64 as f64).log2())).floor() as usize;

    // Handle negative numbers
    if input < zero {
        result.push('-');
        input = -input;
    }

    // Special case for zero
    if input == zero {
        result.push(CHARS.chars().next().unwrap());
        return result;
    }

    // Find highest integer place using only multiplication
    let mut integer_places = vec![BigNum::from(1)];
    while integer_places.last().unwrap() * &big_base <= input {
        let next_place = integer_places.last().unwrap() * &big_base;
        integer_places.push(next_place);
    }

    // Process integer part using precomputed places
    let mut remainder = input.clone();
    for place in integer_places.iter().rev() {
        let mut digit: usize = 0;
        // Find digit through repeated subtraction (avoids division)
        while remainder >= *place {
            remainder = remainder - place;
            digit += 1;
        }
        result.push(CHARS.chars().nth(digit).unwrap());
    }

    // Process fractional part using multiplication
    if remainder > zero {
        result.push('.');
        let mut fractional = remainder;
        let mut precision = 0;

        while fractional > zero && precision < target_digits {
            fractional = fractional * &big_base;
            let digit = fractional.get_integer_part_ref();
            fractional = fractional - &digit;
            result.push(CHARS.chars().nth(digit.to_u64() as usize).unwrap());
            precision += 1;
        }
    }

    result
}

impl FromStr for BigNum {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parse(s, Base::Decimal))
    }
}

impl Display for BigNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", encode(self.clone(), Base::Decimal))
    }
}
