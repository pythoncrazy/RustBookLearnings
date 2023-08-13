use std::io;

fn main() {
    println!("What is the best opening move?");
    println!("Make it in standard PGN Notation (aka 3. Ke2#)");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line :(");
    println!("Imagine guessing: {guess}"); // The Best opening is 1.Ke2!!
    if guess == "1.Ke2"{
        println!("Based");
    } //The
}
