use dialoguer::{theme::ColorfulTheme, Input};

pub fn valid_int<T>(prompt: &str) -> T
where
    T: std::str::FromStr,
{
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.trim().parse::<T>().is_ok() {
                Ok(())
            } else {
                Err("Please enter a valid number")
            }
        })
        .interact_text()
        .unwrap()
        .trim()
        .parse()
        .expect("Failed to parse from string to integer")
}