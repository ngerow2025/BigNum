
use std::{str::FromStr, fmt::{self, Display}, error::Error};

use crate::bigNum::BigNum;

const CHARS: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ+/";

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
    let decimal = input.find('.');
    if decimal.is_some() {
        //get the part before the decimal point
        let mut before = input[..decimal.unwrap()].to_string();
        //get the part after the decimal point
        let mut after = input[decimal.unwrap() + 1..].to_string();
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
    let mut place = big_base.clone();
    while place <= input {
        place = place * &big_base;
    }
    place = place / &big_base;

    if input < BigNum::from(0) {
        result.push('-');
        input = -input;
    }
    let mut decimal_point = false;
    while input > BigNum::from(0) {
        let leftover = &input % &place;
        let digit = (&input - &leftover) / &place;
        result.push(CHARS.chars().nth(digit.to_u64() as usize).unwrap());
        input = leftover;
        place = place / &big_base;
        if !decimal_point && place < BigNum::from(1) {
            result.push('.');
            decimal_point = true;
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