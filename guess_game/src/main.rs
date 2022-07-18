// rust 默认会导入一个 prelude
// String 就在 prelude 中

use std::io;     // 不在prelude里头

fn main() {
    println!("Guessing number");

    let mut guess = String::new();  // 默认变量不可变
    // 加上 mut 可变
    // String 默认UTF-8编码

    io::stdin().read_line(&mut guess).expect("无法读取");
    // 返回 io::Result （是一个enums类型，有两个 OK 与 Err
    // io 也可以换为 std::io
    // 这里有一个 & 取地址，也需要 mut 标记为可变

    println!("guessing number is {}", guess);
}
