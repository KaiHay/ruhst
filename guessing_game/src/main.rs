use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {


    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    'dog: loop {
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue 'dog,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("you got it!");
                break 'dog;
            }
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
        }
    }
}
