fn fill_vec(vec: &mut Vec<i32>) {  // 接收可变引用
    vec.push(88);  // 直接修改传入的 Vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let mut vec0 = vec![22, 44, 66];  // 必须是可变的
        fill_vec(&mut vec0);  // 传递可变引用
        assert_eq!(vec0, vec![22, 44, 66, 88]);
    }
}
