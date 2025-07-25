fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

fn main() {
    // "blue" 是字符串字面量，类型为 &str → 使用 string_slice
    string_slice("blue");

    // "red".to_string() 生成 String → 使用 string
    string("red".to_string());

    // String::from("hi") 生成 String → 使用 string
    string(String::from("hi"));

    // "rust is fun!".to_owned() 生成 String → 使用 string
    string("rust is fun!".to_owned());

    // "nice weather".into() 对字符串字面量调用 into() 生成 String → 使用 string
    string("nice weather".into());

    // format! 宏返回 String → 使用 string
    string(format!("Interpolation {}", "Station"));

    // &String::from("abc")[0..1] 是 &str（String 切片的引用）→ 使用 string_slice
    string_slice(&String::from("abc")[0..1]);

    // "  hello there ".trim() 返回 &str → 使用 string_slice
    string_slice("  hello there ".trim());

    // "Happy Monday!".replace(...) 返回 String → 使用 string
    string("Happy Monday!".replace("Mon", "Tues"));

    // "mY sHiFt KeY iS sTiCkY".to_lowercase() 返回 String → 使用 string
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
