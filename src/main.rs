use std::cmp::Ordering;
use rand::Rng;
use std::io;

fn main() {
    println!("Guessing Game");

    let secret_number= rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Input a number: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failure to read line");

        let mut guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("You have guessed {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

}