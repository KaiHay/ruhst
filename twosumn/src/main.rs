use std::collections::HashMap;
use crate::seerch::findme::{found_me, syntaxtice};

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
pub fn better_two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32>{
    let mut hash_map : HashMap<i32, i32> = HashMap::new();

    for (i, &num) in nums.iter().enumerate(){
        let complement = target - num;
        if let Some(&numb) = hash_map.get(&complement) {
            return vec![numb as i32, i as i32]
        }
        hash_map.insert(num,i as i32);
    }
    vec![]


}
fn main() {
    let nums = vec![1, 2, 3];
    println!("{:#?}", two_sum(&nums, 3));
    println!("{:#?}", better_two_sum(&nums, 3));
    found_me();
    syntaxtice();
}
