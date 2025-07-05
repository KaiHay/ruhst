use std::collections::HashMap;

pub mod seerch;

pub fn better_two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&numb) = hash_map.get(&complement) {
            return vec![numb as i32, i as i32];
        }
        hash_map.insert(num, i as i32);
    }
    vec![]
}

//Trait utility

// mod help {
//     trait Drawable {
//         fn draw(&self);
//         fn draw_border(&self) {
//             println!("+-----+");
//             self.draw();
//             println!("+-----+");
//         }
//     }
// }
