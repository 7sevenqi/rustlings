fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // 测试 2^0 = 1
        assert_eq!(power_of_2(0), 1);
        // 测试 2^1 = 2
        assert_eq!(power_of_2(1), 2);
        // 测试 2^3 = 8
        assert_eq!(power_of_2(3), 8);
        // 测试 2^10 = 1024
        assert_eq!(power_of_2(10), 1024);
    }
}
