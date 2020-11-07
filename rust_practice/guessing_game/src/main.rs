use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("the secret number is {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Fail to read line");

    let guess: u32 = guess.trim().parse().expect("please type number");

    println!("You guessed {}", guess);

    
}
