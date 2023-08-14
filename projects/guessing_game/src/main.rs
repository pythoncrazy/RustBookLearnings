use std::io;
use rand::Rng;

fn main() {
    println!("Guess a Random Number!");
    println!("The number is between 1 and 100!");
    let secret_number = rand::thread_rng().gen_range(1..=101);

    println!("The secret number is {secret_number}");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Put a vaild number pls!");
    
    let guess: u32 = match guess.trim().parse().expect("Please type a number!");
    
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    
}
