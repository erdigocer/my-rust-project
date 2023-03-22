use rand::Rng;
use std::io;

use crate::erdi_test::secret;

mod erdi_test;
mod struct_example;

fn main() {
    secret();

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
