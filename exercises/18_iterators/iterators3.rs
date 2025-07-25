#[derive(Debug, PartialEq, Eq)]
enum DivisionError {
    DivideByZero,
    IntegerOverflow,
    NotDivisible,
}

fn divide(a: i64, b: i64) -> Result<i64, DivisionError> {
    // 处理除以零的情况
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }
    
    // 处理整数溢出的情况：i64::MIN / -1
    if a == i64::MIN && b == -1 {
        return Err(DivisionError::IntegerOverflow);
    }
    
    // 计算商和余数
    let quotient = a / b;
    let remainder = a % b;
    
    // 检查是否整除
    if remainder != 0 {
        return Err(DivisionError::NotDivisible);
    }
    
    Ok(quotient)
}

fn result_with_list() -> Result<Vec<i64>, DivisionError> {
    let numbers = [27, 297, 38502, 81];
    numbers.into_iter()
        .map(|n| divide(n, 27))
        .collect()
}

fn list_of_results() -> Vec<Result<i64, DivisionError>> {
    let numbers = [27, 297, 38502, 81];
    numbers.into_iter()
        .map(|n| divide(n, 27))
        .collect()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_integer_overflow() {
        assert_eq!(divide(i64::MIN, -1), Err(DivisionError::IntegerOverflow));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(divide(81, 6), Err(DivisionError::NotDivisible));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(result_with_list().unwrap(), [1, 11, 1426, 3]);
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(list_of_results(), [Ok(1), Ok(11), Ok(1426), Ok(3)]);
    }
}
