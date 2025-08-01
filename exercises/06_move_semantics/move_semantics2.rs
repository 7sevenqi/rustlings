fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(88);
    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        // 创建 vec0 的克隆并传递给 fill_vec
        let vec1 = fill_vec(vec0.clone());

        assert_eq!(vec0, [22, 44, 66]);  // vec0 仍然有效
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
