enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // 实现 transformer 函数
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        input.into_iter().map(|(s, cmd)| {
            match cmd {
                Command::Uppercase => s.to_uppercase(),
                Command::Trim => s.trim().to_string(),
                Command::Append(n) => s + &"bar".repeat(n),
            }
        }).collect()
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // 引入 transformer 函数和 Command 枚举
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
