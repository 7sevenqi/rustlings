#[derive(PartialEq, Debug)]
enum List {
    // 使用 Box 包装 List 以解决递归类型的大小问题
    Cons(i32, Box<List>),
    Nil,
}

// 创建一个空的 cons 列表
fn create_empty_list() -> List {
    List::Nil
}

// 创建一个非空的 cons 列表
fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
