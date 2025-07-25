fn main() {
    let my_option: Option<()> = None;
    if let Some(value) = my_option {
        // 使用直接变量引用的格式化语法
        println!("{value:?}");
    } else {
        println!("my_option is None");
    }

    let my_arr = &[-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {my_arr:?}");

    // 指定 Vec 的元素类型
    let my_empty_vec: Vec<i32> = Vec::new();
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}
