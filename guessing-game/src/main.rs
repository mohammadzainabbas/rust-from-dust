use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select, Sort};
use dialoguer::console::Term;

mod validate;
mod guessing_games;

fn play_games(theme: &ColorfulTheme) {

    let games = &[
        "Guess the number",
        "Guess the word",
    ];

    Select::with_theme(theme)
        .with_prompt("Pick an item")
        .items(games)
        .interact()
        .unwrap();



}

fn quit() {
    println!("\n{}\n", "Nevermind then :(".bright_red().on_black());
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
        Some(true) => play_games(&theme),
        Some(false) => quit(),
        None => println!("Hello"),
    }




    // let term = Term::buffered_stderr();

    // Confirm::with_theme(&theme)
    //     .with_prompt(format!("{}", "Do you want to continue?".bright_yellow()))
    //     .interact_on(&term)
    //     .unwrap();

    // let _: String = Input::with_theme(&theme)
    //     .with_prompt("Your name")
    //     .interact_on(&term)
    //     .unwrap();

    // Select::with_theme(&theme)
    //     .with_prompt("Pick an item")
    //     .items(items)
    //     .interact_on(&term)
    //     .unwrap();

    // MultiSelect::with_theme(&theme)
    //     .with_prompt("Pick some items")
    //     .items(items)
    //     .interact_on(&term)
    //     .unwrap();

    // Sort::with_theme(&theme)
    //     .with_prompt("Order these items")
    //     .items(items)
    //     .interact_on(&term)
    //     .unwrap();


}
