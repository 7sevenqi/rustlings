#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // 对 optional_point 取引用，避免所有权转移
    match &optional_point {
        Some(p) => println!("Co-ordinates are {},{}", p.x, p.y),
        None => println!("No coordinates provided"),
    }

    // 此时 optional_point 仍有效，可以正常打印
    println!("{optional_point:?}");
}
