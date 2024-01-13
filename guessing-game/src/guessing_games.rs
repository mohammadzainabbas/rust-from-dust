use std::cmp::Ordering;
use rand::Rng;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Input};

use crate::{validate, utils};

#[allow(unused)]
pub fn guess_the_number(theme: &ColorfulTheme) {
    utils::colored_println(format!("🚀 Let's play '{}' game.", "Guess the number!".cyan().bold()).bright_yellow().italic());

    let mut min: u32;
    let mut max: u32;

    loop {
        min = validate::valid_int(theme, format!("Pick {} threshold: ", "min".cyan().bold()));
        max = validate::valid_int(theme, format!("Pick {} threshold: ", "max".cyan().bold()));

        match min.cmp(&max) {
            Ordering::Less => break,
            _ => {
                utils::colored_println(format!("{} should be less than {}.", "min".cyan().bold(), "max".cyan().bold()).bright_yellow().italic());
                continue;
            }
        }
    }
    
    let guess: u32 = validate::valid_int(theme, format!("Guess a number b/w {} & {}: ", min.to_string().cyan().bold(), max.to_string().cyan().bold()));
    // let guess: u32 = validate::valid_int("Guess a number");


    // let _name: String = Input::with_theme(&ColorfulTheme::default()).with_prompt("Enter your name: ").interact_text().unwrap();
    // let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    // println!("Your secret number is {}", secret_number);

    // loop {
    //     let guess: u32 = Input::with_theme(&ColorfulTheme::default())
    //         .with_prompt("Guess a number: ")
    //         .validate_with(|input: &String| -> Result<(), &str> {
    //             if input.trim().parse::<u32>().is_ok() {
    //                 Ok(())
    //             } else {
    //                 Err("Please enter a valid number")
    //             }
    //         })
    //         .interact_text()
    //         .unwrap()
    //         .trim()
    //         .parse()
    //         .expect("Failed to parse to u32");

    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Less"),
    //         Ordering::Greater => println!("Greater"),
    //         Ordering::Equal => println!("Eq"),
    //     }
    // }
    
    // let mut guess = String::new();
    // io::stdin().read_line(&mut guess).expect("Unable to read line from stdin");
    

    // println!("Your guessed value is: {}", guess);
}

#[allow(unused)]
pub fn guess_the_word(theme: &ColorfulTheme) {
    println!("Let's play 'Guess the word!' game");
}
