// 使用泛型 T 使 Wrapper 能包装任意类型
struct Wrapper<T> {
    value: T,
}

// 为泛型结构体 Wrapper<T> 实现方法
impl<T> Wrapper<T> {
    // 构造函数接受任意类型 T 的值
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // 可选：在此处进行实验
    let int_wrapper = Wrapper::new(100);
    let str_wrapper = Wrapper::new("hello");
    println!("int: {}, str: {}", int_wrapper.value, str_wrapper.value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
