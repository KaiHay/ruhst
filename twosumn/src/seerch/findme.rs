use std::collections::HashMap;

use crate::better_two_sum;

pub struct Look {
    stupid: Vec<i32>,
    target: i32,
}

pub fn found_me() {
    println!("Found me! Hahaha")
}

pub fn syntaxtice() {
    let mector = vec![1, 7, 10, 20, 4, 5, 7, 244, 56];
    let see_this = Look {
        stupid: mector,
        target: 245,
    };
    let meal = better_two_sum(&see_this.stupid, see_this.target);

    println!("{:?}", meal);
}
fn is_prime(n: usize) -> bool {
    // 0 and 1 are not prime
    if n < 2 {
        return false;
    }
    // 2 and 3 are prime
    if n <= 3 {
        return true;
    }
    // eliminate even numbers
    if n % 2 == 0 {
        return false;
    }
    // only check odd divisors up to √n
    let limit = (n as f64).sqrt() as usize;
    let mut i = 3;
    while i <= limit {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}
pub fn sieve(n: usize) -> Vec<usize> {
    //loop through all numbers while less than n
    //check if its prime, then push into ans
    let mut ans = Vec::new();
    for i in 2..n {
        if is_prime(i) {
            ans.push(i)
        }
    }
    return ans;
}

pub fn is_anagram(s: String, t: String) -> bool {
    //declare hashmap which holds each character in the string
    //loop through first word adding keys
    //loop through second subtracting and if the key goes negative return false

    let mut hm: HashMap<char, i64> = HashMap::new();
    if s.len() != t.len() {
        return false;
    }
    for i in s.chars() {
        let key = hm.entry(i).or_insert(0);
        *key += 1;
    }
    for j in t.chars() {
        let key = hm.entry(j).or_insert(-1);
        *key -= 1;
        if *key < 0 {
            return false;
        }
    }
    true
}
