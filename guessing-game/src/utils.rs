use colored::ColoredString;

pub fn colored_print(text: ColoredString, newline: bool) {

    println!("{}", if newline {format!("\n{}\n", text)} else {text})
}