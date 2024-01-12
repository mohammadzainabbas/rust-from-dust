use std::io;
use rand::Rng;
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Your secret number is {}", secret_number);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Unable to read line from stdin");

    println!("Your guessed value is: {}", guess);
}
