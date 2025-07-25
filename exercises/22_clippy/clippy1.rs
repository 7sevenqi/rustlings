fn main() {
    // 使用标准库提供的精确 π 常量，类型与 radius 保持一致
    let pi = std::f32::consts::PI; // 类型为 f32，与 radius 匹配
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    // 注意：f32 的精度可能不足以精确表示五位小数
    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}
