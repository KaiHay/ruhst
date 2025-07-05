use twosumn::better_two_sum;

use crate::seerch::{
    findme::{found_me, sieve, syntaxtice},
    lifetraits::longest_with_an_announcement,
};


pub mod seerch;

pub fn two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
        let compl = &target - &nums[i];
        for j in 1..nums.len() {
            if j != i {
                if nums[j] == compl {
                    ans.push(i.try_into().unwrap());
                    ans.push(j.try_into().unwrap());
                    return ans;
                }
            }
        }
    }
    ans
}


fn main() {
    let nums = vec![1, 2, 3];
    println!("{:#?}", two_sum(&nums, 3));
    println!("{:#?}", better_two_sum(&nums, 3));
    found_me();
    syntaxtice();

    let primes = sieve(200);
    println!("Primes <= 50 : \n {:?}", primes);

    let a: String = "poopy".to_string();
    let b = ("duplex").to_string();

    let longest = longest_with_an_announcement(&a, &b, "lol");
    println!("Longest is: {}", longest);
}

