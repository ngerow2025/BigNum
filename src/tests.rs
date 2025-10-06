use crate::BigNum;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use lazy_static::lazy_static;

lazy_static! {
    static ref POSSIBLE_VALS: Vec<BigNum> = {
        let mut rng = StdRng::seed_from_u64(42); // replace 42 with any seed
        vec![
            BigNum::from(0),
            BigNum::from(1),
            BigNum::from(100),
            BigNum::from(1000000000000000000u64),
            BigNum::from(-1),
            BigNum::from(-100),
            BigNum::from(-1000000000000000000i64),
            BigNum::from(i64::MAX),
            BigNum::from(i64::MIN),
            generate_random_bignum(&mut rng, 3, 0),
            -generate_random_bignum(&mut rng, 3, 0),
            generate_random_bignum(&mut rng, 3, -100),
            -generate_random_bignum(&mut rng, 3, -100),
        ]
    };
}

#[expect(dead_code)] // this warning is from being used in the lazy_static block only
fn generate_random_bignum<R: Rng>(rng: &mut R, digits: usize, exp: i32) -> BigNum {
    let mut result = BigNum::new();
    let a = BigNum::from(1) + BigNum::from(u64::MAX);
    for _ in 0..digits {
        result = result * &a + BigNum::from(rng.gen::<u64>());
    }
    result = result * a.pow(exp.into());
    result
}

#[test]
fn new() {
    let num = BigNum::new();
    assert_eq!(num.to_u64(), 0);
}

#[test]
fn default() {
    let num = BigNum::default();
    assert_eq!(num.to_u64(), 0);
}

#[test]
fn from_u64() {
    test_from_u64(0);
    test_from_u64(1);
    test_from_u64(100);
    test_from_u64(10000000000000000000);
    test_from_u64(u64::MAX);
    test_from_u64(u64::MIN);
}

#[cfg(test)]
fn test_from_u64(n: u64) {
    let num = BigNum::from(n);
    assert_eq!(num.to_u64(), n);
}

#[test]
fn from_i64() {
    test_from_i64(0);
    test_from_i64(1);
    test_from_i64(100);
    test_from_i64(1000000000000000000);
    test_from_i64(-1);
    test_from_i64(-100);
    test_from_i64(-1000000000000000000);
    test_from_i64(i64::MAX);
    test_from_i64(i64::MIN);
}

#[cfg(test)]
fn test_from_i64(n: i64) {
    let num = BigNum::from(n);

    if n >= 0 {
        assert_eq!(num.to_u64(), n as u64);
    }

    assert_eq!(num < 0.into(), n < 0);
}

#[test]
fn from_i32() {
    test_from_i32(0);
    test_from_i32(1);
    test_from_i32(100);
    test_from_i32(1000000000);
    test_from_i32(-1);
    test_from_i32(-100);
    test_from_i32(-1000000000);
    test_from_i32(i32::MAX);
    test_from_i32(i32::MIN);
}

#[cfg(test)]
fn test_from_i32(n: i32) {
    let num = BigNum::from(n);
    if n >= 0 {
        assert_eq!(num.to_u64(), n as u64);
    }
    assert_eq!(num < 0.into(), n < 0);
}

#[test]
fn from_u32() {
    test_from_u32(0);
    test_from_u32(1);
    test_from_u32(100);
    test_from_u32(1000000000);
    test_from_u32(u32::MAX);
    test_from_u32(u32::MIN);
}

#[cfg(test)]
fn test_from_u32(n: u32) {
    let num = BigNum::from(n);
    assert_eq!(num.to_u64(), n as u64);
}

#[test]
fn commutative_property_of_addition() {
    // a + b = b + a

    for a in POSSIBLE_VALS.iter() {
        for b in POSSIBLE_VALS.iter() {
            test_commutative_property_of_addition(a, b);
        }
    }
}

#[cfg(test)]
fn test_commutative_property_of_addition(a: &BigNum, b: &BigNum) {
    let result1 = a + b;
    let result2 = b + a;
    assert_eq!(
        result1, result2,
        "Commutative property of addition failed: {:?} + {:?} != {:?} + {:?}",
        a, b, b, a
    );
}

#[test]
fn associative_property_of_addition() {
    // (a + b) + c = a + (b + c)
    for a in POSSIBLE_VALS.iter() {
        for b in POSSIBLE_VALS.iter() {
            for c in POSSIBLE_VALS.iter() {
                test_associative_property_of_addition(a, b, c);
            }
        }
    }
}

#[cfg(test)]
fn test_associative_property_of_addition(a: &BigNum, b: &BigNum, c: &BigNum) {
    let result1 = (a + b) + c;
    let result2 = a + (b + c);
    assert_eq!(
        result1, result2,
        "Associative property of addition failed: ({:?} + {:?}) + {:?} != {:?} + ({:?} + {:?})",
        a, b, c, a, b, c
    );
}

#[test]
fn additive_identity() {
    // a + 0 = a
    for a in POSSIBLE_VALS.iter() {
        test_additive_identity(a);
    }
}

#[cfg(test)]
fn test_additive_identity(a: &BigNum) {
    let result = a + BigNum::from(0);
    assert_eq!(
        result, *a,
        "Additive identity failed: {:?} + 0 != {:?}",
        a, a
    );
}

#[test]
fn additive_inverse() {
    // a + (-a) = 0
    for a in POSSIBLE_VALS.iter() {
        test_additive_inverse(a);
    }
}

#[cfg(test)]
fn test_additive_inverse(a: &BigNum) {
    let neg = -a;
    let result = a + &neg;
    assert_eq!(
        result,
        BigNum::from(0),
        "Additive inverse failed: {:?} + (-{:?}) != 0",
        a,
        a
    );
}

#[test]
fn distributive_property() {
    // a * (b + c) = (a * b) + (a * c)
    for a in POSSIBLE_VALS.iter() {
        println!("a: {:?}", a);
        for b in POSSIBLE_VALS.iter() {
            println!("  b: {:?}", b);
            for c in POSSIBLE_VALS.iter() {
                test_distributive_property(a, b, c);
            }
        }
    }
}

#[cfg(test)]
fn test_distributive_property(a: &BigNum, b: &BigNum, c: &BigNum) {
    let result1 = a * (b + c);
    let result2 = (a * b) + (a * c);
    assert_eq!(
        result1, result2,
        "Distributive property failed: {:?} * ({:?} + {:?}) != ({:?} * {:?}) + ({:?} * {:?})",
        a, b, c, a, b, a, c
    );
}

#[test]
fn equality() {
    //a == a
    for a in POSSIBLE_VALS.iter() {
        test_equality(a);
    }
}

#[cfg(test)]
fn test_equality(a: &BigNum) {
    assert_eq!(*a, *a, "a: {:?}", a);
}

#[test]
fn anti_reflexive_property_of_inequality() {
    // x < x is false
    // x > x is false
    for a in POSSIBLE_VALS.iter() {
        test_anti_reflexive_property_of_inequality(a);
    }
}

#[cfg(test)]
fn test_anti_reflexive_property_of_inequality(a: &BigNum) {
    assert!(
        !(a < a),
        "Anti-reflexive property of inequality failed: {:?} < {:?} is true",
        a,
        a
    );
    assert!(
        !(a > a),
        "Anti-reflexive property of inequality failed: {:?} > {:?} is true",
        a,
        a
    );
}

#[test]
fn anti_symmetry_property_of_inequality() {
    // if x < y, then y < x is false
    // if x > y, then y > x is false
    for a in POSSIBLE_VALS.iter() {
        for b in POSSIBLE_VALS.iter() {
            test_anti_symmetry_property_of_inequality(a, b);
        }
    }
}

#[cfg(test)]
fn test_anti_symmetry_property_of_inequality(a: &BigNum, b: &BigNum) {
    if a == b {
        return;
    }

    let less_than = a < b;
    let greater_than = a > b;
    let less_than_symmetric = b < a;
    let greater_than_symmetric = b > a;

    assert_eq!(
        less_than_symmetric, !less_than,
        "Anti-symmetry property failed: {:?} < {:?} but {:?} < {:?}",
        a, b, b, a
    );
    assert_eq!(
        greater_than_symmetric, !greater_than,
        "Anti-symmetry property failed: {:?} > {:?} but {:?} > {:?}",
        a, b, b, a
    );
}

#[test]
fn transitive_property_of_inequality() {
    // if x < y and y < z, then x < z
    // if x > y and y > z, then x > z
    for a in POSSIBLE_VALS.iter() {
        for b in POSSIBLE_VALS.iter() {
            for c in POSSIBLE_VALS.iter() {
                test_transitive_property_of_inequality(a, b, c);
            }
        }
    }
}

#[cfg(test)]
fn test_transitive_property_of_inequality(a: &BigNum, b: &BigNum, c: &BigNum) {
    let less_than1 = a < b;
    let less_than2 = b < c;
    let less_than3 = a < c;
    let greater_than1 = a > b;
    let greater_than2 = b > c;
    let greater_than3 = a > c;

    if less_than1 && less_than2 {
        assert!(
            less_than3,
            "Transitive property failed: {:?} < {:?} and {:?} < {:?} but {:?} !< {:?}",
            a, b, b, c, a, c
        );
    }

    if greater_than1 && greater_than2 {
        assert!(
            greater_than3,
            "Transitive property failed: {:?} > {:?} and {:?} > {:?} but {:?} !> {:?}",
            a, b, b, c, a, c
        );
    }
}

#[test]
fn addition_property_of_inequality() {
    // if x < y, then x + z < y + z
    // if x > y, then x + z > y + z
    for a in POSSIBLE_VALS.iter() {
        for b in POSSIBLE_VALS.iter() {
            for c in POSSIBLE_VALS.iter() {
                test_addition_property_of_inequality(a, b, c);
            }
        }
    }
}

#[cfg(test)]
fn test_addition_property_of_inequality(a: &BigNum, b: &BigNum, c: &BigNum) {
    let less_than = a < b;
    let greater_than = a > b;
    let less_than_add = a + c < b + c;
    let greater_than_add = a + c > b + c;

    assert_eq!(
        less_than_add, less_than,
        "Failed addition property of inequality test: {:?} < {:?} but {:?} + {:?} < {:?} + {:?}",
        a, b, a, c, b, c
    );
    assert_eq!(
        greater_than_add, greater_than,
        "Failed addition property of inequality test: {:?} > {:?} but {:?} + {:?} > {:?} + {:?}",
        a, b, a, c, b, c
    );
}

#[test]
fn subtraction_property_of_inequality() {
    // if x < y, then x - z < y - z
    // if x > y, then x - z > y - z
    for a in POSSIBLE_VALS.iter() {
        for b in POSSIBLE_VALS.iter() {
            for c in POSSIBLE_VALS.iter() {
                test_subtraction_property_of_inequality(a, b, c);
            }
        }
    }
}

#[cfg(test)]
fn test_subtraction_property_of_inequality(a: &BigNum, b: &BigNum, c: &BigNum) {
    let less_than = a < b;
    let greater_than = a > b;
    let less_than_sub = a - c < b - c;
    let greater_than_sub = a - c > b - c;

    assert_eq!(
        less_than_sub, less_than,
        "Failed subtraction property of inequality test: {:?} < {:?} but {:?} - {:?} < {:?} - {:?}",
        a, b, a, c, b, c
    );
    assert_eq!(
        greater_than_sub, greater_than,
        "Failed subtraction property of inequality test: {:?} > {:?} but {:?} - {:?} > {:?} - {:?}",
        a, b, a, c, b, c
    );
}

#[test]
fn multiplication_property_of_inequality() {
    // if x < y and z > 0, then x * z < y * z
    // if x < y and z < 0, then x * z > y * z
    // if x > y and z > 0, then x * z > y * z
    // if x > y and z < 0, then x * z < y * z
    for a in POSSIBLE_VALS.iter() {
        for b in POSSIBLE_VALS.iter() {
            for c in POSSIBLE_VALS.iter() {
                test_multiplication_property_of_inequality(a, b, c);
            }
        }
    }
}

#[cfg(test)]
fn test_multiplication_property_of_inequality(a: &BigNum, b: &BigNum, c: &BigNum) {
    let less_than = a < b;
    let greater_than = a > b;
    let less_than_mul = a * c < b * c;
    let greater_than_mul = a * c > b * c;

    if c > &BigNum::from(0) {
        assert_eq!(
            less_than_mul, less_than,
            "Multiplication property failed: {:?} < {:?} but {:?} * {:?} !< {:?} * {:?}",
            a, b, a, c, b, c
        );
        assert_eq!(
            greater_than_mul, greater_than,
            "Multiplication property failed: {:?} > {:?} but {:?} * {:?} !> {:?} * {:?}",
            a, b, a, c, b, c
        );
    } else if c < &BigNum::from(0) {
        assert_eq!(
            less_than_mul, greater_than,
            "Multiplication property failed: {:?} < {:?} but {:?} * {:?} !> {:?} * {:?}",
            a, b, a, c, b, c
        );
        assert_eq!(
            greater_than_mul, less_than,
            "Multiplication property failed: {:?} > {:?} but {:?} * {:?} !< {:?} * {:?}",
            a, b, a, c, b, c
        );
    }
}

#[test]
fn inequality_sanity_test() {
    // a < a + b if b > 0
    // a > a + b if b < 0

    for a in POSSIBLE_VALS.iter() {
        for b in POSSIBLE_VALS.iter() {
            test_inequality_sanity_test(a, b);
        }
    }
}

#[cfg(test)]
fn test_inequality_sanity_test(a: &BigNum, b: &BigNum) {
    let less_than = a < &(a + b);
    let greater_than = a > &(a + b);

    if b > &BigNum::from(0) {
        assert!(
            less_than,
            "Failed inequality sanity test: {:?} < {:?} + {:?}",
            a, a, b
        );
    } else if b < &BigNum::from(0) {
        assert!(
            greater_than,
            "Failed inequality sanity test: {:?} > {:?} + {:?}",
            a, a, b
        );
    }
}
