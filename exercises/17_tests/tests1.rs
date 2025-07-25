fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // 导入外部模块的所有内容，包括 is_even 函数
    use super::*;

    #[test]
    fn you_can_assert() {
        // 测试偶数情况
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(is_even(-4));
        
        // 测试奇数情况
        assert!(!is_even(1));
        assert!(!is_even(-3));
    }
}
