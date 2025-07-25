trait AppendBar {
    fn append_bar(self) -> Self;
}

// 为 Vec<String> 实现 AppendBar trait
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push("Bar".to_string()); // 向向量中添加 "Bar"
        self                           // 返回修改后的向量
    }
}

fn main() {
    // 可选：在此处进行实验
    let mut vec = vec![String::from("Hello")].append_bar();
    println!("{:?}", vec); // 输出: ["Hello", "Bar"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
