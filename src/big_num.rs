use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

#[derive(Debug, Clone, Default)]
pub struct BigNum {
    parts: Vec<u64>,
    exp: i32,
    neg: bool,
}

impl BigNum {
    pub fn new() -> BigNum {
        BigNum {
            parts: Vec::new(),
            exp: 0,
            neg: false,
        }
    }
    pub fn truncate(self, n: usize) -> BigNum {
        let mut result = self;
        while result.parts.len() > n {
            result.parts.pop();
        }
        result
    }
    //print out each part in binary including a space between each part or a decimal point where appropriate
    pub fn to_string_binary(&self) -> String {
        let mut result = String::new();
        for i in (0..self.parts.len()).rev() {
            if i == self.exp as usize {
                result.push('.');
            }
            result.push_str(&format!("{:064b}", self.parts[i]));
            if i != 0 {
                result.push(' ');
            }
        }
        if self.neg {
            result.insert(0, '-');
        }
        result
    }

    pub fn get_precision(&self) -> usize {
        self.parts.len()
    }
}

impl From<u64> for BigNum {
    fn from(num: u64) -> Self {
        let parts = vec![num];
        BigNum {
            parts,
            exp: 0,
            neg: false,
        }
    }
}

impl From<i64> for BigNum {
    fn from(num: i64) -> Self {
        let parts = vec![num.abs_diff(0)];
        BigNum {
            parts,
            exp: 0,
            neg: num < 0,
        }
    }
}

impl From<i32> for BigNum {
    fn from(num: i32) -> Self {
        let parts = vec![(num as i64).unsigned_abs()];
        BigNum {
            parts,
            exp: 0,
            neg: num < 0,
        }
    }
}

impl From<u32> for BigNum {
    fn from(num: u32) -> Self {
        let parts = vec![num as u64];
        BigNum {
            parts,
            exp: 0,
            neg: false,
        }
    }
}

impl BigNum {
    //1234 * 10^-2 = 12.34
    //12 * 10^-4 = 0.0012
    pub fn get_integer_part(self) -> BigNum {
        if self.exp >= 0 {
            return self;
        }
        if self.parts.len() <= -self.exp as usize {
            return self;
        }

        let mut res = self.clone();
        while res.exp < 0 {
            if res.parts.is_empty() {
                return BigNum::new();
            }
            res.parts.remove(0);
            res.exp += 1;
        }
        res
    }

    pub fn get_integer_part_ref(&self) -> BigNum {
        if self.exp >= 0 {
            return self.clone();
        }
        if self.parts.len() < -self.exp as usize {
            return self.clone();
        }

        let mut res = self.clone();
        while res.exp < 0 {
            if res.parts.is_empty() {
                return BigNum::new();
            }
            res.parts.remove(0);
            res.exp += 1;
        }
        res.compact();
        res
    }

    //1234 * 10^-2 = 12.34
    //
    pub fn get_decimal_part(self) -> BigNum {
        if self.exp >= 0 {
            return BigNum::new();
        }
        if self.parts.len() <= -self.exp as usize {
            return BigNum::new();
        }
        let mut res = self.clone();
        let n = res.exp.abs();
        for _ in 0..n {
            res.parts.pop();
        }
        res
    }
}

impl Add for BigNum {
    type Output = BigNum;
    fn add(self, rhs: Self) -> Self::Output {
        // -a + b = b - a
        if self.neg && !rhs.neg {
            //copy self and make it positive
            let mut pos_self = self.clone();
            pos_self.neg = false;
            return rhs - pos_self;
        }
        // a + -b = a - b
        if !self.neg && rhs.neg {
            //copy rhs and make it positive
            let mut pos_rhs = rhs.clone();
            pos_rhs.neg = false;
            return self - pos_rhs;
        }
        // -a + -b
        //-(a + b)
        if self.neg && rhs.neg {
            let mut a = self.clone();
            let mut b = rhs.clone();
            a.neg = false;
            b.neg = false;
            let mut result = a + b;
            result.neg = true;
            return result;
        }

        // a + b
        let mut a = self;
        let mut b = rhs;
        align(&mut a, &mut b);
        let mut result = BigNum::new();
        //set the size of results parts to be the same as a + 1
        result.parts.resize(a.parts.len(), 0);
        let mut carry = false;
        for i in 0..a.parts.len() {
            let sum;
            let tmp_carry1;
            let tmp_carry2;
            (sum, tmp_carry1) = a.parts[i].overflowing_add(b.parts[i]);
            (result.parts[i], tmp_carry2) = sum.overflowing_add(carry as u64);
            carry = tmp_carry1 || tmp_carry2;
        }
        if carry {
            result.parts.push(1);
        }
        result.exp = a.exp;
        result.compact();
        result
    }
}

impl Add for &BigNum {
    type Output = BigNum;
    fn add(self, rhs: Self) -> Self::Output {
        self.clone() + rhs.clone()
    }
}

impl Add<&BigNum> for BigNum {
    type Output = BigNum;
    fn add(self, rhs: &BigNum) -> Self::Output {
        self + rhs.clone()
    }
}

impl Add<BigNum> for &BigNum {
    type Output = BigNum;
    fn add(self, rhs: BigNum) -> Self::Output {
        self.clone() + rhs
    }
}

impl BigNum {
    pub fn compact(&mut self) {
        //remove any leading and trailing zeros
        while self.parts.last() == Some(&0) {
            self.parts.pop();
        }
        while self.parts.first() == Some(&0) {
            self.parts.remove(0);
            self.exp += 1;
        }
    }
}

//aligns both the exponents of a and b and the length of parts of a and b
//this function does not change the value of a or b, just the representation
fn align(a: &mut BigNum, b: &mut BigNum) {
    a.compact();
    b.compact();
    while a.exp != b.exp {
        if a.exp > b.exp {
            a.parts.insert(0, 0);
            a.exp -= 1;
        } else {
            b.parts.insert(0, 0);
            b.exp -= 1;
        }
    }
    while a.parts.len() != b.parts.len() {
        if a.parts.len() > b.parts.len() {
            b.parts.push(0);
        } else {
            a.parts.push(0);
        }
    }
}

impl Sub for BigNum {
    type Output = BigNum;
    fn sub(self, rhs: Self) -> Self::Output {
        // a - -b
        // a + b
        if !self.neg && rhs.neg {
            let mut pos_rhs = rhs;
            pos_rhs.neg = false;
            return self + pos_rhs;
        }

        // -a - b
        // -(a + b)
        if self.neg && !rhs.neg {
            let mut pos_self = self;
            pos_self.neg = false;
            let mut result = pos_self + rhs;
            result.neg = true;
            return result;
        }

        // -a - -b
        // -a + b
        // b - a
        if self.neg && rhs.neg {
            let mut pos_rhs = rhs;
            pos_rhs.neg = false;
            let mut pos_self = self;
            pos_self.neg = false;
            return pos_rhs - pos_self;
        }

        // a - b
        //if a == b, return 0
        if self == rhs {
            return BigNum::new();
        }

        //if a < b, return -(b - a)
        if self < rhs {
            let mut result = rhs - self;
            result.neg = true;
            return result;
        }

        // a - b
        let mut a = self;
        let mut b = rhs;
        align(&mut a, &mut b);
        let mut result = BigNum::new();
        //set the size of results parts to be the same as a
        result.parts.resize(a.parts.len(), 0);
        let mut borrow = false;
        for i in 0..a.parts.len() {
            let diff;
            let tmp_borrow1;
            let tmp_borrow2;
            (diff, tmp_borrow1) = a.parts[i].overflowing_sub(b.parts[i]);
            (result.parts[i], tmp_borrow2) = diff.overflowing_sub(borrow as u64);
            borrow = tmp_borrow1 || tmp_borrow2;
        }
        debug_assert!(!borrow);
        result.exp = a.exp;
        result.compact();
        result
    }
}

impl Sub for &BigNum {
    type Output = BigNum;
    fn sub(self, rhs: Self) -> Self::Output {
        self.clone() - rhs.clone()
    }
}

impl Sub<&BigNum> for BigNum {
    type Output = BigNum;
    fn sub(self, rhs: &BigNum) -> Self::Output {
        self - rhs.clone()
    }
}

impl Sub<BigNum> for &BigNum {
    type Output = BigNum;
    fn sub(self, rhs: BigNum) -> Self::Output {
        self.clone() - rhs
    }
}

impl PartialEq for BigNum {
    fn eq(&self, other: &Self) -> bool {
        let mut a = self.clone();
        let mut b = other.clone();
        align(&mut a, &mut b);
        //remove any leading zeros
        while a.parts.last() == Some(&0) {
            a.parts.pop();
        }
        while b.parts.last() == Some(&0) {
            b.parts.pop();
        }
        if a.parts.is_empty() && b.parts.is_empty() {
            return true;
        }
        if self.neg != other.neg {
            return false;
        }
        if a.parts.len() != b.parts.len() {
            return false;
        }
        for i in 0..a.parts.len() {
            if a.parts[i] != b.parts[i] {
                return false;
            }
        }
        true
    }
}

impl Eq for BigNum {}

impl PartialOrd for BigNum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigNum {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        if self.neg && !other.neg {
            return Ordering::Less;
        }
        if !self.neg && other.neg {
            return Ordering::Greater;
        }

        let mut a = self.clone();
        let mut b = other.clone();
        align(&mut a, &mut b);
        if a.parts.len() > b.parts.len() {
            if a.neg {
                return Ordering::Less;
            } else {
                return Ordering::Greater;
            }
        }
        if a.parts.len() < b.parts.len() {
            if a.neg {
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }

        for i in (0..a.parts.len()).rev() {
            if a.parts[i] > b.parts[i] {
                if a.neg {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }
            if a.parts[i] < b.parts[i] {
                if a.neg {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            }
        }
        Ordering::Equal
    }
}

impl Neg for BigNum {
    type Output = BigNum;
    fn neg(self) -> Self::Output {
        let mut result = self;
        result.neg = !result.neg;
        result
    }
}

impl Neg for &BigNum {
    type Output = BigNum;
    fn neg(self) -> Self::Output {
        self.clone().neg()
    }
}

impl Mul for BigNum {
    type Output = BigNum;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = BigNum::new();
        let mut a = self;
        let mut b = rhs;
        align(&mut a, &mut b);

        //set the size of result.parts to the length of the digits of a and b summed
        result.parts.resize(a.parts.len() + b.parts.len(), 0);

        for i in 0..a.parts.len() {
            for j in 0..b.parts.len() {
                let (prod, mut carry) = widening_mul(a.parts[i], b.parts[j]);
                let mut k = i + j;
                //add prod, check for carry and add it to carry
                let (tmp, tmp_carry) = result.parts[k].overflowing_add(prod);
                result.parts[k] = tmp;
                carry += tmp_carry as u64;
                k += 1;
                while carry > 0 {
                    if k >= result.parts.len() {
                        result.parts.push(0);
                    }
                    let (tmp, tmp_carry) = result.parts[k].overflowing_add(carry);
                    result.parts[k] = tmp;
                    carry = tmp_carry as u64;
                    k += 1;
                }
            }
        }
        result.exp = a.exp + b.exp;
        result.neg = a.neg ^ b.neg;
        result.compact();
        result
    }
}

impl Mul for &BigNum {
    type Output = BigNum;
    fn mul(self, rhs: Self) -> Self::Output {
        self.clone() * rhs.clone()
    }
}

impl Mul<&BigNum> for BigNum {
    type Output = BigNum;
    fn mul(self, rhs: &BigNum) -> Self::Output {
        self * rhs.clone()
    }
}

impl Mul<BigNum> for &BigNum {
    type Output = BigNum;
    fn mul(self, rhs: BigNum) -> Self::Output {
        self.clone() * rhs
    }
}

fn widening_mul(a: u64, b: u64) -> (u64, u64) {
    //convert into u128 and multiply
    let prod = (a as u128) * (b as u128);
    //get the lower 64 bits of prod
    let lower = (prod & 0xFFFFFFFFFFFFFFFF) as u64;
    //get the upper 64 bits of prod
    let upper = (prod >> 64) as u64;
    (lower, upper)
}

impl Div for BigNum {
    type Output = BigNum;
    fn div(self, rhs: Self) -> Self::Output {
        let mut result = BigNum::new();
        let default_precision = self.parts.len() + rhs.parts.len();
        let mut a = self;
        let mut b = rhs;
        align(&mut a, &mut b);
        if b == BigNum::from(0) {
            panic!("Divide by zero");
        }

        let mut offset = 0;

        for _ in 0..default_precision {
            if b > a {
                a.parts.insert(0, 0);
                offset += 1;
            }
            let mut tmp = b.clone();
            let mut count = 0;
            let mut sub_count = 0;
            let exponent = BigNum::from(u64::MAX) + BigNum::from(1);
            let mut did_iterate = false;
            while &tmp * &exponent < a {
                did_iterate = true;
                tmp.exp += 1;
                count += 1;
            }

            if did_iterate {
                tmp.exp -= 1;
                count -= 1;
            }

            while &tmp * BigNum::from(2) <= a {
                tmp = tmp * BigNum::from(2);
                sub_count += 1;
            }

            //count is the multible of b that is closest to a
            //repeatedly subtract tmp from a until a is less than tmp and each time add a bignum with exp = count to result
            while a >= tmp {
                a = a - &tmp;
                let mut tmp_result = BigNum::from(1);
                tmp_result.exp = count - offset;
                for _ in 0..sub_count {
                    tmp_result = tmp_result * BigNum::from(2);
                }
                result = result + tmp_result;
                //check if needed precision has been reached
                result.compact();
                if result.parts.len() > default_precision {
                    //round off the last part and remove it
                    //if the most significant bit is 1, add 1 to the last part
                    //the representation is least significant u64 first
                    let least_significant_part = result.parts.first().unwrap();
                    let rounding_bit = least_significant_part & 0x8000000000000000;
                    //remove the last part
                    result.parts.remove(0);
                    if rounding_bit != 0 {
                        break;
                    }
                }
                //we have exactly divided a by b
                if a == BigNum::from(0) {
                    break;
                }
            }
        }
        result.neg = a.neg ^ b.neg;
        result.compact();

        result
    }
}

impl Div for &BigNum {
    type Output = BigNum;
    fn div(self, rhs: Self) -> Self::Output {
        self.clone() / rhs.clone()
    }
}

impl Div<&BigNum> for BigNum {
    type Output = BigNum;
    fn div(self, rhs: &BigNum) -> Self::Output {
        self / rhs.clone()
    }
}

impl Div<BigNum> for &BigNum {
    type Output = BigNum;
    fn div(self, rhs: BigNum) -> Self::Output {
        self.clone() / rhs
    }
}

impl Rem for BigNum {
    type Output = BigNum;
    fn rem(self, rhs: Self) -> Self::Output {
        let mut tmp = self.clone();
        while tmp >= rhs.clone() {
            tmp = tmp - rhs.clone();
        }
        tmp
    }
}

impl Rem<&BigNum> for &BigNum {
    type Output = BigNum;
    fn rem(self, rhs: &BigNum) -> Self::Output {
        self.clone() % rhs.clone()
    }
}

impl Rem<&BigNum> for BigNum {
    type Output = BigNum;
    fn rem(self, rhs: &BigNum) -> Self::Output {
        self % rhs.clone()
    }
}

impl Rem<BigNum> for &BigNum {
    type Output = BigNum;
    fn rem(self, rhs: BigNum) -> Self::Output {
        self.clone() % rhs
    }
}

impl BigNum {
    pub fn pow(self, exp: Self) -> Self {
        if exp.neg {
            return BigNum::from(1) / (self.pow(-exp));
        }
        if exp == BigNum::from(0) {
            return BigNum::from(1);
        }
        if exp == BigNum::from(1) {
            return self;
        }
        let int_part = exp.clone().get_integer_part();
        let mut result = BigNum::from(1);
        let mut i = BigNum::from(0);
        while i < int_part {
            result = result * self.clone();
            i = i + BigNum::from(1);
        }
        let frac_part = exp.get_decimal_part();
        if frac_part == BigNum::from(0) {
            return result;
        }
        let mut numerator = frac_part.clone();
        numerator.exp = 0;
        let mut denomonator = BigNum::from(1);
        for _ in 0..(-frac_part.exp) {
            denomonator = denomonator * (BigNum::from(u64::MAX) + BigNum::from(1));
        }
        let gcd = Self::gcd(numerator.clone(), denomonator.clone());
        numerator = numerator / gcd.clone();
        denomonator = denomonator / gcd;

        let mut tmp_res = self.pow(numerator.clone());
        tmp_res = tmp_res.root(denomonator);

        result = result * tmp_res;
        result
    }

    pub fn gcd(a: Self, b: Self) -> Self {
        assert!(a.clone().get_decimal_part() == BigNum::from(0));
        assert!(b.clone().get_decimal_part() == BigNum::from(0));
        let mut a = a;
        let mut b = b;
        if a < b {
            std::mem::swap(&mut a, &mut b);
        }
        while b != BigNum::from(0) {
            let tmp = b.clone();
            b = a % b;
            a = tmp;
        }
        a
    }

    pub fn root(self, rhs: Self) -> Self {
        //rhs is the root
        if rhs.neg || rhs.clone().get_decimal_part() != BigNum::from(0) {
            return self.pow(BigNum::from(1) / rhs);
        }
        //use newton's method to find the root
        let mut x = self.clone();
        let factor1 = (rhs.clone() - BigNum::from(1)) / rhs.clone();
        let factor2 = self.clone() / rhs.clone();

        for _ in 0..100 {
            x = &factor1 * &x + &factor2 / &x.pow(rhs.clone() - BigNum::from(1));
        }
        x
    }

    pub fn to_u64(&self) -> u64 {
        let mut a = self.clone();
        a.compact();
        if a.parts.is_empty() {
            return 0;
        }
        assert!(a.parts.len() == 1);
        assert!(!a.neg);
        assert!(a.exp == 0);
        a.parts[0]
    }
}
