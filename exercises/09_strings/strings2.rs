fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // 不要修改此行

    if is_a_color_word(&word) { // 将 String 转换为 &str
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}
