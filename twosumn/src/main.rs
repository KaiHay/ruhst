pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
        let compl = &target-&nums[i];
        for j in 1..nums.len(){
            if j!=i {
                if nums[j] == compl {
                    ans.push(i.try_into().unwrap());
                    ans.push(j.try_into().unwrap());
                    return ans
                }
            }
        }
    }
    ans
}
fn main() {
    let nums = vec![1, 2, 3];
    println!("{:?}", two_sum(nums, 3))
}
