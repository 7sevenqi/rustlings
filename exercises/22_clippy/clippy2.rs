fn main() {
    let mut res = 42;
    let option = Some(12);

    // 使用 if let 替代 for 循环
    if let Some(x) = option {
        res += x;
    }

    println!("{res}"); // 输出 54
}
