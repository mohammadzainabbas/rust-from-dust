use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select, Sort};
use dialoguer::console::Term;

mod validate;
mod guessing_games;

fn main() {

    println!("{}", "Let's play some guessing games".bright_blue().on_black());

    let items = &[
        "Guess the number",
        "Guess the word",
    ];

    let term = Term::buffered_stderr();
    let theme = ColorfulTheme::default();

    Confirm::with_theme(&theme)
        .with_prompt("Do you want to continue?")
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
