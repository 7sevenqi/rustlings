fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        output.push(element * 2); // 自动解引用并乘以2
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // 示例：将每个元素加1
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // 使用迭代器映射，将每个元素乘以2
    input
        .iter()
        .map(|element| element * 2) // 闭包中直接乘以2
        .collect()
}

fn main() {
    // 可在此处测试函数
    let input = [2, 4, 6];
    println!("Loop result: {:?}", vec_loop(&input));
    println!("Map result: {:?}", vec_map(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
