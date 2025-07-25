#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // 使用 Arc 包装向量，使其可在多线程间安全共享
    let shared_numbers = Arc::new(numbers);

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // 克隆 Arc 以创建新的引用，共享底层数据的所有权
        let child_numbers = Arc::clone(&shared_numbers);

        let handle = thread::spawn(move || {
            // 过滤出符合当前 offset 的元素并求和
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    // 等待所有线程完成
    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
