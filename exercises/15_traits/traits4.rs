trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// 使用 trait 对象，接受任何实现了 Licensed trait 的引用
fn compare_license_types(software1: &dyn Licensed, software2: &dyn Licensed) -> bool {
    software1.licensing_info() == software2.licensing_info()
}

fn main() {
    // 可选：在此处进行实验
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        assert!(compare_license_types(&SomeSoftware, &OtherSoftware));
    }

    #[test]
    fn compare_license_information_backwards() {
        assert!(compare_license_types(&OtherSoftware, &SomeSoftware));
    }
}
