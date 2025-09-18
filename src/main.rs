use std::io;
use rand::Rng;

fn main() {
    println!("Guessing Game");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Input a number: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failure to read line");

    println!("You have guessed {guess}");
}