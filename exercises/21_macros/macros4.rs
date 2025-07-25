#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }; // 添加分号分隔分支
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();        // 调用第一个分支
    my_macro!(7777);    // 调用第二个分支
}
