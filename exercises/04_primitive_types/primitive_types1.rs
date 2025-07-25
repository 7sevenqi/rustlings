fn main() {
    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    // 定义 is_evening 变量，值为 is_morning 的否定
    let is_evening = !is_morning;
    if is_evening {
        println!("Good evening!");
    }
}
