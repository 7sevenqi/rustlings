#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

#[derive(Debug)]
enum Message {
    // 结构体样式的变体，包含 width 和 height 字段
    Resize { width: u64, height: u64 },
    // 元组样式的变体，包含一个 Point 结构体
    Move(Point),
    // 元组样式的变体，包含一个 String
    Echo(String),
    // 元组样式的变体，包含三个 u32 类型的值（RGB 颜色）
    ChangeColor(u32, u32, u32),
    // 单元样式的变体，无关联数据
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {
            width: 10,
            height: 30,
        },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
