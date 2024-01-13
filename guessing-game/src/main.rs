use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect};
use dialoguer::console::{Key, Term};

mod utils;
mod validate;
mod guessing_games;

fn play_games(theme: &ColorfulTheme) {

    const TERMINAL_TITLE: &str = "🚀 Menu - Guessing Games";

    let games: &[(&str, fn(&ColorfulTheme))] = &[
        ("Guess the number", guessing_games::guess_the_number),
        ("Guess the word", guessing_games::guess_the_word),
    ];

    let term = Term::stdout();
    term.set_title(TERMINAL_TITLE);
    
    loop {
        let selection = FuzzySelect::with_theme(theme)
            .with_prompt(format!("{} {}:", "Pick a game".bright_yellow(), "(use fuzzy search)".cyan()))
            .default(0)
            .items(&games.iter().map(|(name, _)| *name).collect::<Vec<_>>())
            .interact()
            .unwrap();
    
        let (game_name, game_fn) = games[selection];
        
        utils::colored_println(format!("{}{}", "You have picked: ".bright_cyan(), game_name.green().bold()).italic());
        game_fn(theme);
        utils::colored_print(format!("\nPress any key to continue. {} to exit!", "Esc".blue().bold()).bright_yellow().italic());
        let key = term.read_key();
        match key.unwrap() {
            Key::Escape => break,
            _ => {
                        term.set_title(TERMINAL_TITLE);
                        // term.clear_screen().expect("Unable to clear screen");
                        continue;
                    },
                }
    }
}

fn quit() {
    utils::colored_println("Nevermind then 🥺".bright_yellow())
}

fn no_answer() {
    utils::colored_println("User did not answer 🥺".red())
}

fn main() {
    
    utils::colored_println("Let's play some guessing games".bright_blue().on_black());
    
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
