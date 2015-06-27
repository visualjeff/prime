#![allow(dead_code)]
#![allow(unused_imports)]

#![feature(test)]
//extern crate rand;
extern crate test;

use test::Bencher;

#[test]
fn is_it_a_prime_number() {
    assert_eq!(true, is_prime(5));
    assert_eq!(true, is_prime_why_not(5));
}

#[test]
fn should_not_be_a_prime_number() {
    assert_eq!(false, is_prime(4));
    assert_eq!(false, is_prime_why_not(4));
}

#[test]
fn zero_is_not_a_prime_number() {
    assert_eq!(false, is_prime(0));
    assert_eq!(false, is_prime_why_not(0));
}

#[test]
fn one_is_not_a_prime_number() {
    assert_eq!(false, is_prime(1));
    assert_eq!(false, is_prime_why_not(1));
}

#[test]
fn prime_factorization_test(){
    let mut vec = Vec::new();
    vec.push(2);
    vec.push(2);
    vec.push(2);
    vec.push(3);
    vec.push(7);
    assert_eq!(vec, prime_factorization(168));
    vec = Vec::new();
    vec.push(2);
    vec.push(3);
    vec.push(17);
    assert_eq!(vec, prime_factorization(102));
}

#[test]
fn get_next_prime_number_test() {
    assert_eq!(79, get_next_prime_number(73));
}

#[test]
fn greatest_common_factor_test() {
    assert_eq!(6, greatest_common_factor(18,24));
}

#[test]
fn least_common_multiple_test() {
    assert_eq!(90, least_common_multiple(18,30));
}

#[test]
fn number_of_primes_test() {
    assert_eq!(494, number_of_primes(1,3500));
}

//#[bench]
//fn bench_is_prime(b: &mut Bencher) {
//    b.iter(|| is_prime(rand::random::<i32>()));
//}

/// Tests for prime numbers.
///
/// # Examples
///
/// ```
/// use primapalooza::is_prime;
/// 
/// println!("{}", primapalooza::is_prime(5));
/// 
pub fn is_prime(x: i32) -> bool {
    let mut start: i64 = 2;
    while start <= (x as f64).sqrt() as i64 {
        if (x as i64) % start < 1 {
            return false;
        }
        start += 1;
    }
    return x > 1;
}

//#[bench]
//fn bench_is_prime_why_not(b: &mut Bencher) {
//      b.iter(|| 
//             for n in (0..10) {
//                test::black_box(is_prime_why_not(n));
//             });
//}

/// Tests for prime numbers and state why its not a prime number.
///
/// # Examples
///
/// ```
/// use primapalooza::is_prime_why_not;
/// 
/// println!("{}", primapalooza::is_prime_why_not(9));
/// 
pub fn is_prime_why_not(x: i32) -> bool {
    let mut start: i64 = 2;
    let mut return_value: bool = true;
    while start <= (x as f64).sqrt() as i64 {
        let m = (x as i64) % start; 
        if m < 1 {
            println!("{} is divisible by {}.  {} * {} = {}", x, start, start, ((x as i64) / start), x);
            return_value = false;
        }
        start += 1;
    }
    if return_value == true { 
        return_value = x > 1;
    }
    return return_value;
}

//#[bench]
//fn bench_get_next_prime_number(b: &mut Bencher) {
//    b.iter(|| 
//           for n in (1..10) {
//               test::black_box(get_next_prime_number(n));
//           });
//}

/// Returns the next available prime number.
///
/// # Examples
///
/// ```
/// use primapalooza::get_next_prime_number;
/// 
/// println!("{}", primapalooza::get_next_prime_number(9));
/// 
pub fn get_next_prime_number(n:i32) -> i32 {
    //println!("{}", n);
    let mut x: i32 = n;
    x += 1;
    while !is_prime(x) {
        x += 1;
    }
    return x;
}


//#[bench]
//fn bench_prime_factorization(b: &mut Bencher) {
//    b.iter(||
//           for n in (1..10) {
//                test::black_box(prime_factorization(n));
//           });
//}

/// Perform a prime factorization.
///
/// # Examples
///
/// ```
/// use primapalooza::prime_factorization;
/// 
/// println!("{:?}", primapalooza::prime_factorization(168));
/// 
pub fn prime_factorization(n:i32) -> Vec<i64> {
    let mut x:i64 = n as i64;
    let mut return_value = Vec::new();
    let mut start:i64 = 2;
    while !is_prime(x as i32) {
        let m = x % start;
        if m < 1 {
            return_value.push(start);
            x = x / start;
        } else {
            let s = start as i32;
            start = get_next_prime_number(s) as i64;                                                                  }
    }
    return_value.push(x);
    return return_value;
}


/// Perform a greatest common factor(gcf) calculation.
///
/// # Examples
///
/// ```
/// use primapalooza::greatest_common_factor;
/// 
/// println!("{}", primapalooza::greatest_common_factor(18, 24));
/// 
pub fn greatest_common_factor(x:i32, y:i32) -> i32 {
    let mut vec_x = prime_factorization(x);
    let mut vec_y = prime_factorization(y);
    vec_x.dedup();
    vec_y.dedup();
    let mut m:Vec<i32> = Vec::new();
    for p in vec_x.iter() {
        for q in vec_y.iter() {
            if p == q {
                m.push(*p as i32);
            }
        }
    }
    let mut return_value = 1;
    for k in &m {
        return_value = return_value * k;    
    }
    return return_value;
}


/// Perform a least common multiple(lcm) calculation.
///
/// # Examples
///
/// ```
/// use primapalooza::least_common_multiple;
/// 
/// println!("{}", primapalooza::least_common_multiple(18, 24));
/// 
pub fn least_common_multiple(x:i32, y:i32) -> i32 {
    let gcf = greatest_common_factor(x, y);
    if x % gcf < 1 {
        (x/gcf) * y
    } else {
        (y/gcf) * x
    }
}


/// Number of prime numbers within a range.
///
/// # Examples
///
/// ```
/// use primapalooza::number_of_primes;
/// 
/// println!("{}", primapalooza::number_of_primes(1, 3500));
/// 
pub fn number_of_primes(x: i32, y: i32) -> i32 {
    let mut z = 0;
    for n in x..y {
        if is_prime(n) {
            z += 1
        }
    }
    return z;
}
