use dialoguer::{theme::ColorfulTheme, Input};
use std::fmt::Debug;

pub fn valid_int<T>(prompt: &String) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: Debug,
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