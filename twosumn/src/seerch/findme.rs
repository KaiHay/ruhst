use std::collections::HashMap;

use crate::better_two_sum;

pub struct Look {
    stupid: Vec<i32>,
    target: i32,
}

pub fn found_me() {
    println!("Found me! Hahaha")
}
//VECTOR PRACTICE -----------------------------------------
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
//HASHMAP PRACTICE-----------------------------------------------------------
pub fn is_anagram(s: String, t: String) -> bool {
    //declare hashmap which holds each character in the string
    //loop through first word adding keys
    //loop through second subtracting and if the key goes negative return false
    if s.len() != t.len() {
        return false;
    }
    let mut hm: HashMap<char, i64> = HashMap::new();

    for i in s.chars() {
        let key = hm.entry(i).or_insert(0);
        //gets the key or inserts the key, returns the mutable reference to the key
        *key += 1; //have to dereference the key to change its value
    }
    for j in t.chars() {
        if let Some(key) = hm.get_mut(&j) {
            //if something declare key, if nothing go to the else 
            *key -= 1;
            if *key < 0 {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

pub fn length_of_longest_substring(s: String) -> i32 {
    //double for loop
    //move pt1 forward until first dupe
    //log max len
    //move pt2 forward

    if s.is_empty() { //is empty is actually a vector function!
        return 0;
    }

    //collect() can take anything iterable, and turn it into a relevant collection
    let chars: Vec<char> = s.chars().collect(); 
    let n = chars.len();
    let mut max_len = 0;
    for i in 0..n {
        let mut seen: HashMap<char, u8> = HashMap::new();
        let mut cur_len = 0;

        for j in i..n {
            let c = chars[j];
            let counter = seen.entry(c).or_insert(0);
            *counter += 1;

            if *counter > 1 {
                break;
            }

            cur_len += 1;
        }

        if cur_len > max_len {
            max_len = cur_len;
        }
    }

    max_len
}
