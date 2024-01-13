pub fn valid_u32() {
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
}