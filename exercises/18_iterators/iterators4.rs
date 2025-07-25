fn factorial(num: u64) -> u64 {
    // 生成 1 到 num 的范围迭代器，计算所有元素的乘积
    (1..=num).product()
}

fn main() {
    // 可选：在此处进行实验
    println!("5! = {}", factorial(5)); // 输出：5! = 120
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
