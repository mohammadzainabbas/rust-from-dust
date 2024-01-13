use colored::ColoredString;

pub fn colored_print(text: ColoredString, newline: bool) {
    println!("\n{}\n", text);
}