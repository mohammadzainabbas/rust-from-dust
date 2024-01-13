use std::cmp::Ordering;
use dialoguer::{Input, FuzzySelect};
use rand::Rng;
use colored::Colorize;
use dialoguer::theme::ColorfulTheme;
use dialoguer::console::{Key, Term};
use crate::{validate, utils};

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
        guess_tries += 1;
        
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                let tries_str = if guess_tries == 1 { "try" } else { "tries" };
                utils::colored_print(format!("\n🎉 Correct guess! You took {} {tries_str} to guess the secret number!", guess_tries.to_string().cyan().bold()).bright_yellow().italic());
                break;
            },
            _ => {
                utils::colored_print("Incorrect guess".bright_red().italic());
                if guess_tries % HINT_AFTER == 0 {
                    utils::colored_print(format!("\n{} Press {} for a hint! Press {} to quit this game! Press any other key to continue...", "Having trouble guessing?".magenta().bold(), "Tab".blue().bold(), "Esc".blue().bold()).bright_yellow().italic());
                    let key = term.read_key();
                    match key.unwrap() {
                        Key::Escape => {
                            utils::colored_print(format!("\n🥺 You gave up after {} tries!", guess_tries.to_string().cyan().bold()).bright_yellow().italic());
                            break;
                        },
                        Key::Tab => {
                            let far: i32 = (guess as i32 - secret_number as i32).abs();
                            utils::colored_println(format!("Your last guess {} is {} numbers far from the actual answer!", guess.to_string().cyan().bold(), far.to_string().cyan().bold()).bright_yellow().italic());
                            continue;
                        },
                        _ => continue,
                    }
                }
            }
        }
    }
}

#[allow(unused)]
pub fn guess_the_word(theme: &ColorfulTheme, term: &Term) {
    utils::colored_println(format!("🚀 Let's play '{}' game.", "Guess the word!".cyan().bold()).bright_yellow().italic());

    // Initialize an empty vector to store the words
    let mut words: Vec<String> = Vec::new();
    const END_WORD: &str = ":q";
    
    loop {
        // Prompt the user for a word
        let word: String = Input::with_theme(theme)
            // .with_prompt("Enter a word (type :q to finish):")        
            .with_prompt(format!("{} {}{}{}", "Enter a word".cyan().bold(), "(type ".bright_yellow(), END_WORD.cyan().bold(), " to finish): ".bright_yellow()))        
            .interact()
            .unwrap();

        // Check if the input is empty, and exit the loop if it is
        if word == END_WORD {
            break;
        }

        if (words.contains(&word)) {
            utils::colored_print(format!("You have entered '{}' already!", word.cyan().bold()).bright_red().italic());
        } else {
            words.push(word); // Add the word to the vector
        }
    }

    words.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase())); // sort

    // Choose a word randomly
    let random_index = rand::thread_rng().gen_range(0..words.len());
    let target_word = &words[random_index];
    
    const ASK_FOR_STOP_AFTER: u32 = 5; // ask if user wants to stop after 5 incorrect guesses
    let mut guess_tries: u32 = 0;

    loop {
        // Ask the user to select a word
        let selected_index = FuzzySelect::with_theme(theme)
            .with_prompt(format!("{} {}:", "Guess a word".bright_yellow(), "(use fuzzy search)".cyan()))
            .items(&words)
            .interact()
            .unwrap();

        let selected_word = &words[selected_index];
        guess_tries += 1;

        // Check if the selected word is correct
        match selected_word.cmp(target_word) {
            Ordering::Equal => {
                let tries_str = if guess_tries == 1 { "try" } else { "tries" };
                utils::colored_print(format!("\n🎉 Correct guess! You took {} {tries_str} to guess '{}'!", guess_tries.to_string().cyan().bold(), target_word.cyan().bold()).bright_yellow().italic());
                break;
            },
            _ => {
                utils::colored_print("Wrong word. Try again!".bright_red().italic());
                if guess_tries % ASK_FOR_STOP_AFTER == 0 {
                    utils::colored_print(format!("\n{} Press {} for a hint! Press {} to quit this game! Press any other key to continue...", "Having trouble guessing?".magenta().bold(), "Tab".blue().bold(), "Esc".blue().bold()).bright_yellow().italic());
                    let key = term.read_key();
                    match key.unwrap() {
                        Key::Escape => {
                            utils::colored_print(format!("\n🥺 You gave up after {} tries!", guess_tries.to_string().cyan().bold()).bright_yellow().italic());
                            break;
                        },
                        Key::Tab => {
                            utils::colored_println(format!("Actual answer is '{}'!", target_word.to_string().cyan().bold()).bright_yellow().italic());
                            continue;
                        },
                        _ => continue,
                    }
                }
            }
        }
    }
}
