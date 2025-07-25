// 直接接受 &mut u32，无需泛型和 AsMut 约束
fn num_sq(arg: &mut u32) {
    *arg *= *arg;
}

fn main() {
    let mut num: u32 = 5;
    num_sq(&mut num);
    println!("Squared: {}", num);  // 输出 25
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        // 需要先解引用 Box 才能获取 &mut u32
        num_sq(&mut *num);
        assert_eq!(*num, 9);
    }
}
