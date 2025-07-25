#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 实现 Default  trait 作为解析失败时的 fallback
impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("John"),
            age: 30,
        }
    }
}

// 实现 From<&str>  trait，用于从字符串解析 Person
impl From<&str> for Person {
    fn from(s: &str) -> Self {
        // 按逗号分割字符串
        let parts: Vec<&str> = s.split(',').collect();
        
        // 检查分割后的部分是否恰好为 2 个（名称和年龄）
        if parts.len() != 2 {
            return Self::default();
        }
        
        let name = parts[0].trim();
        let age_str = parts[1].trim();
        
        // 检查名称是否为空
        if name.is_empty() {
            return Self::default();
        }
        
        // 解析年龄为 u8 类型
        match age_str.parse::<u8>() {
            Ok(age) => Self {
                name: name.to_string(),
                age,
            },
            Err(_) => Self::default(), // 解析失败返回默认值
        }
    }
}

// 主函数，用于演示功能
fn main() {
    // 使用 from 方法转换
    let p1 = Person::from("Mark,20");
    println!("{p1:?}");

    // 由于实现了 From，可自动使用 Into 转换
    let p2: Person = "Gerald,70".into();
    println!("{p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn test_bad_age() {
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,dog");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
