use crate::better_two_sum;

pub struct Look {
    stupid: Vec<i32>,
    target: i32,
}

pub fn found_me() {
    println!("Found me! Hahaha")
}

pub fn syntaxtice() {
    let mector = vec![1,7,10,20,4,5,7,244,56];
    let see_this=Look {
        stupid: mector,
        target: 245,
    };
    let meal = better_two_sum(&see_this.stupid, see_this.target);

    println!("{:?}",meal);
}