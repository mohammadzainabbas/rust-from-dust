use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select, Sort};
use dialoguer::console::Term;

mod validate;
mod guessing_games;

fn start_games() {

    let games = &[
        "Guess the number",
        "Guess the word",
    ];

    


}

fn main() {

    println!("\n{}\n", "Let's play some guessing games".bright_blue().on_black());
    
    let theme = ColorfulTheme::default();

    match Confirm::with_theme(&theme)
        .with_prompt(format!("{}", "Do you want to continue?".bright_yellow()))
        .default(true)
        .interact_opt()
        .unwrap() 
    {
        Some(true) => start_games(),
        Some(false) => start_games(),
        None => start_games(),
    }




    let term = Term::buffered_stderr();

    Confirm::with_theme(&theme)
        .with_prompt(format!("{}", "Do you want to continue?".bright_yellow()))
        .interact_on(&term)
        .unwrap();

    let _: String = Input::with_theme(&theme)
        .with_prompt("Your name")
        .interact_on(&term)
        .unwrap();

    Select::with_theme(&theme)
        .with_prompt("Pick an item")
        .items(items)
        .interact_on(&term)
        .unwrap();

    MultiSelect::with_theme(&theme)
        .with_prompt("Pick some items")
        .items(items)
        .interact_on(&term)
        .unwrap();

    Sort::with_theme(&theme)
        .with_prompt("Order these items")
        .items(items)
        .interact_on(&term)
        .unwrap();


}
