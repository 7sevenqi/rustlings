// 将宏定义移动到 main 函数之前
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!(); // 现在宏已经被定义，可以正常调用
}
