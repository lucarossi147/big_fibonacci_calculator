use std::collections::HashMap;
use num_bigint::BigUint;
use tailcall::tailcall;
use rug::ops::Pow;
use rug::Float;

pub fn fibo(i: u128) -> u128 {
    match i {
        n if n < 2 => n,
        n => fibo(n-1) + fibo(n-2)
    }
}

pub fn memoized_fibo(i: u32) -> BigUint {
    fn _fibo(i: u32, memo: &mut HashMap<u32, BigUint>) -> BigUint {
        let v = memo.get(&i);
        match v {
            Some(res) => res.clone(),
            None => {
                let f_2 = i-2;
                let fibo_2 = _fibo(f_2, memo);
                memo.insert(f_2, fibo_2.clone());
                let f_1 = i-1;
                let fibo_1 = _fibo(f_1, memo);
                memo.insert(f_1, fibo_1.clone());
                fibo_1 + fibo_2
            }
        }
    }
    _fibo(i, &mut HashMap::from_iter(vec![(0,BigUint::ZERO), ((1,BigUint::new(vec![1])))]))
}

pub fn fibo_tail_rec(n: u32) -> BigUint {
    #[tailcall]
    fn _fibo(n: u32, f1:BigUint, f2: BigUint) -> BigUint {
        match n {
            _ if n < 2 => f2,
            n => _fibo(n-1, f2.clone(), f1 + f2)
        }
    }
    _fibo(n, BigUint::ZERO, BigUint::new(vec![1]))
}


pub fn fibo_iterative(n: u32) -> BigUint {
    let mut f1 = BigUint::ZERO;
    let mut f2 = BigUint::new(vec![1]);
    for _ in 1..n {
        let tmp = f1 + f2.clone();
        f1 = f2;
        f2 = tmp;
    }
    f2
}

pub fn fibo_formula(n: u32) -> BigUint {
    let sqrt_5 = f64::sqrt(5.0);
    let phi = Float::with_val(53, 1.0 + (sqrt_5/2.0));
    let elevated: Float = phi.pow(n as f64) / sqrt_5;
    println!("{:?}", elevated);
    BigUint::ZERO
}

#[test]
fn fibo_test_0() {
    assert_eq!(0, fibo(0))
}

#[test]
fn fibo_test_1() {
    assert_eq!(1, fibo(1));
}

#[test]
fn fibo_test_2() {
    assert_eq!(1, fibo(2));
}

#[test]
fn fibo_test_5() {
    assert_eq!(5, fibo(5));
}

#[test]
fn fibo_test_30() {
    assert_eq!(832040, fibo(30));
}

#[test]
fn memo_fibo_test_0() {
    assert_eq!(BigUint::ZERO, memoized_fibo(0));
}

#[test]
fn memo_fibo_test_1() {
    println!("{:?}", memoized_fibo(1));
}

#[test]
fn memo_fibo_test_30() {
    println!("{:?}", memoized_fibo(1));
    // assert_eq!(832040, memoized_fibo(30));
}

#[test]
fn memo_fibo_test_50() {
    println!("{:?}", memoized_fibo(1));
    // assert_eq!(12586269025, memoized_fibo(50));
}

#[test]
fn memo_fibo_test_100() {
    println!("{:?}", memoized_fibo(20_000));
    // assert_eq!(354224848179261915075, memoized_fibo(100));
}

#[test]
fn first_hundred() {
    for i in 0..100 {
        println!("{:?}", memoized_fibo(i));
    }
}


#[test]
fn first_hundred_iter() {
    for i in 0..100 {
        println!("{:?}", fibo_iterative(i));
    }
}

#[test]
fn first_hundred_tail_rec() {
    for i in 0..100 {
        println!("{:?}", fibo_tail_rec(i));
    }
}

#[test]
fn first_hundred_fibo_formula() {
    for i in 0..100 {
        println!("{:?}", fibo_formula(i));
    }
}