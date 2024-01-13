// use std::io;
use rand::Rng;
use dialoguer::{theme::ColorfulTheme, Input};
fn main() {
    println!("Let's play 'Guess the number!' game");

    let name: String = Input::with_theme(&ColorfulTheme::default()).with_prompt("Enter your name: ").interact_text().unwrap();
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    loop {
        let guess: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Guess a number: ")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.trim().parse::<u32>().is_ok() {
                    Ok(())
                } else {
                    Err("Please enter a valid number")
                }


                // let _num: u32 = input.trim().parse().expect("Please enter a valid number");
                // if input.trim().parse() {
                //     Ok(())
                // } else {
                //     Err("This is not a mail address")
                // }
            })
            .interact_text()
            .unwrap();
        let guess: u32 = guess.trim().parse().expect("Please enter a valid number");

    }

    println!("Your name is {}", name);

    // println!("Please input your guess:");

    
    // let mut guess = String::new();
    // io::stdin().read_line(&mut guess).expect("Unable to read line from stdin");
    
    println!("Your secret number is {}", secret_number);
    println!("Your guessed value is: {}", guess);
}
