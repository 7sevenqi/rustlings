fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// 修复返回类型为 i64，并确保函数体返回一个值
fn sale_price(price: i64) -> i64 {
    if is_even(price) {
        price - 10  // 隐式返回表达式结果（无分号）
    } else {
        price - 3   // 隐式返回表达式结果（无分号）
    }
}

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price)); // 输出：48
}
