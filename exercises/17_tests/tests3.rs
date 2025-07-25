struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height must be positive");
        }
        Rectangle { width, height }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10);  // 验证宽度
        assert_eq!(rect.height, 20); // 验证高度
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height must be positive")]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10); // 应触发 panic
    }

    #[test]
    #[should_panic(expected = "Rectangle width and height must be positive")]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10); // 应触发 panic
    }
}
