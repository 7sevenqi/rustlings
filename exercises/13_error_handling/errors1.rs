// 修改函数返回类型为 Result<String, String>
fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // 返回包含错误信息的 Err 变体
        Err("Empty names aren't allowed".to_string())
    } else {
        // 返回包含成功值的 Ok 变体
        Ok(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        // 使用 as_deref() 将 Result<String, String> 转换为 Result<&str, &str>
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        // 使用 as_ref().map_err(|e| e.as_str()) 将错误类型转换为 &str
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}
