use colored::ColoredString;

pub fn colored_print(text: ColoredString) {
    println!("{}", text);
}

pub fn colored_println(text: ColoredString) {
    println!("\n{}\n", text);
}
