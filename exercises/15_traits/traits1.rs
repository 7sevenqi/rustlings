trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    fn append_bar(self) -> Self {
        // 使用 format! 或直接 push_str 方法追加 "Bar"
        self + "Bar"
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {s}"); // 输出 "s: FooBar"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), "FooBar");
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(String::from("").append_bar().append_bar(), "BarBar");
    }
}
