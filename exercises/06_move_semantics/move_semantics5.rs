#![allow(clippy::ptr_arg)]

// 不应获取所有权，因此参数使用引用
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 应获取所有权，因此参数不使用引用
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data); // 传递引用，不转移所有权

    string_uppercase(data); // 传递所有权
}
