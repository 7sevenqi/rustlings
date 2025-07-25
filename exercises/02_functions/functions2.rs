fn call_me(num: usize) { // 添加类型注解 `usize`
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3); // 调用函数，传入整数参数
}
