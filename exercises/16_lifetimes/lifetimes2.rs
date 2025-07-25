fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");  // 将 string2 移到外部作用域
    let result = longest(&string1, &string2);  // 此时 string2 的生命周期足够长

    println!("The longest string is '{result}'");  // 使用 result 时 string2 仍有效
}
