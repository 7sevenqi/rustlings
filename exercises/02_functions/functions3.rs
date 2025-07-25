fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3); // 传入 u8 类型的参数（字面量默认类型为 i32，但在这种上下文中会自动转换为 u8）
}
