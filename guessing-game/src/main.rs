use std::io;
use rand::Rng;
use dialoguer::{theme::{ColorfulTheme, self}, Input};
fn main() {
    println!("Guess the number!");

    let name = Input::with_theme(&ColorfulTheme::default()).with_prompt("Enter your name: ").interact_text().unwrap();

    println!("Your name is {}", name);

    println!("Please input your guess:");

    let secret_number = rand::thread_rng().gen_range(1..101);
    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Unable to read line from stdin");
    
    println!("Your secret number is {}", secret_number);
    println!("Your guessed value is: {}", guess);
}
