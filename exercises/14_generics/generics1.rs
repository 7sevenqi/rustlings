fn main() {
    // 为向量添加类型注解，指定元素类型为 i16
    let mut numbers: Vec<i16> = Vec::new();

    // 无需修改以下代码
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
