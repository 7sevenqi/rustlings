fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let mut result = String::new();
            result.push(first.to_uppercase().next().unwrap_or(first));
            result.extend(chars);
            result
        }
    }
}

fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter()
        .map(|s| capitalize_first(s))
        .collect()
}

fn capitalize_words_string(words: &[&str]) -> String {
    words.iter()
        .map(|s| capitalize_first(s))
        .collect()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
