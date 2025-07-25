fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);  // 使用 y 后，释放对 x 的可变借用
        
        let z = &mut x;  // 此时可以创建新的可变引用
        z.push(13);      // 使用 z
        
        assert_eq!(x, [42, 13]);
    }
}
