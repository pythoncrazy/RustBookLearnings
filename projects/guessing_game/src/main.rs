use std::io;
use rand::Rng;

fn main() {
    println!("Guess a Random Number!");
    println!("The number is between 1 and 100!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Put a vaild number pls!");
    println!("Your guess was: {guess}");
    
}
