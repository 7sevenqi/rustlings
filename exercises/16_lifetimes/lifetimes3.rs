// 为结构体添加生命周期参数 'a，表明其引用字段必须至少和 'a 一样长
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
