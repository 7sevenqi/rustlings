macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // 正确的宏调用（注意宏名后没有空格，直接跟!和括号）
    my_macro!();
}
