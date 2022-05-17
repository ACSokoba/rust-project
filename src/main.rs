// use ferris_says::say;
use rand::Rng;

use std::io;
use std::io::{stdout, BufWriter};
fn main() {
    helloWorld();
    // ferris_says();
    // guess();
    randomNumberGuess();
}

fn helloWorld() {
    println!("Hello, world!");
}

fn ferris_says() {
    let stdout = stdout();
    let message = String::from("Hello fellow rustaceans !");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    ferris_says::say(message.as_bytes(), width, &mut writer).unwrap();
}

fn guess() {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

fn randomNumberGuess() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
