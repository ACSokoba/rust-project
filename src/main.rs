use ferris_says::say;
use std::io;
use std::io::{stdin, stdout, BufWriter};
fn main() {
    println!("Hello, world!");
    let stdout = stdout();
    let message = String::from("Hello fellow rustaceans !");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
