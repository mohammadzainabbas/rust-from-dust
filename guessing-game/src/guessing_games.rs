use std::cmp::Ordering;
use rand::Rng;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme};
use dialoguer::console::{Key, Term};
use crate::{validate, utils};

#[allow(unused)]
pub fn guess_the_number(theme: &ColorfulTheme, term: &Term) {
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

    let secret_number: u32 = rand::thread_rng().gen_range(min..=max);
    const HINT_AFTER: u32 = 5; // show hint after 5 incorrect guesses
    let mut guess_tries: u32 = 0;

    loop {
        let guess: u32 = validate::valid_int(theme, format!("Guess a number b/w {} & {}: ", min.to_string().cyan().bold(), max.to_string().cyan().bold()));
        
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                utils::colored_println(format!("🎉 Correct guess! You took {} tries to guess the secret number!", guess_tries.to_string().cyan().bold()).bright_yellow().italic());
                break;
            },
            _ => {
                utils::colored_println("Incorrect guess".bright_red().italic());
                guess_tries += 1;
                if guess_tries % HINT_AFTER == 0 {
                    utils::colored_print(format!("\nHaving trouble guessing? Press {} for a hint! Press any other key to continue.", "Alt".blue().bold()).bright_yellow().italic());
                    let key = term.read_key();
                    match key.unwrap() {
                        Key::Alt => {
                            let far: i32 = (guess as i32 - secret_number as i32).abs();
                            utils::colored_println(format!("Your last guess {} is {} digits far from the actual answer!", guess.to_string().cyan().bold(), far.to_string().cyan().bold()).bright_yellow().italic());
                            continue;
                        },
                        _ => continue,
                    }
                }
            }
        }
    }
    
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
pub fn guess_the_word(theme: &ColorfulTheme, term: &Term) {
    println!("Let's play 'Guess the word!' game");
}
