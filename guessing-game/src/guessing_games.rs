use std::cmp::Ordering;
use rand::Rng;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input};

use crate::validate;

#[allow(unused)]
pub fn guess_the_number(theme: &ColorfulTheme) {

    let min: u32 = validate::valid_int(format!("Pick {} threshold: ", "min".cyan().bold()));
    let max: u32 = validate::valid_int(format!("Pick {} threshold: ", "max".cyan().bold()));

    


    let _guess: u32 = validate::valid_int("Guess a number");

    println!("Let's play 'Guess the number!' game");

    let _name: String = Input::with_theme(&ColorfulTheme::default()).with_prompt("Enter your name: ").interact_text().unwrap();
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    println!("Your secret number is {}", secret_number);

    loop {
        let guess: u32 = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Guess a number: ")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.trim().parse::<u32>().is_ok() {
                    Ok(())
                } else {
                    Err("Please enter a valid number")
                }
            })
            .interact_text()
            .unwrap()
            .trim()
            .parse()
            .expect("Failed to parse to u32");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => println!("Eq"),
        }
    }
    
    // let mut guess = String::new();
    // io::stdin().read_line(&mut guess).expect("Unable to read line from stdin");
    

    // println!("Your guessed value is: {}", guess);
}

#[allow(unused)]
pub fn guess_the_word(theme: &ColorfulTheme) {
    println!("Let's play 'Guess the word!' game");
}
