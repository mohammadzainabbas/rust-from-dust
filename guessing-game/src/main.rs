use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect};

mod utils;
mod validate;
mod guessing_games;

fn play_games(theme: &ColorfulTheme) {
    
    let games = &[
        "Guess the number",
        "Guess the word",
    ];

    let selection = FuzzySelect::with_theme(theme)
        .with_prompt(format!("{} {}:", "Pick a game".bright_yellow(), "(use fuzzy search)".cyan()))
        .default(0)
        .items(&games[..])
        .interact()
        .unwrap();
    
    utils::colored_print(format!("You have picked: {}", games[selection].green()).bold())
    
}

fn quit() {
    utils::colored_print("Nevermind then 🥺".bright_yellow())
}

fn no_answer() {
    utils::colored_print("User did not answer 🥺".red())
}

fn main() {

    println!("\n{}\n", "Let's play some guessing games".bright_blue().on_black());
    
    let theme = ColorfulTheme::default();

    let confirmation = Confirm::with_theme(&theme)
        .with_prompt(format!("{}", "Do you want to continue?".bright_yellow()))
        .default(true)
        .interact_opt()
        .unwrap();

    match confirmation
    {
        Some(true) => play_games(&theme),
        Some(false) => quit(),
        None => no_answer(),
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
