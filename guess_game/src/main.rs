// rust 默认会导入一个 prelude
// String 就在 prelude 中

use std::io;     // 不在prelude里头
use rand::Rng;
use std::cmp::Ordering;     // 枚举类型

fn main() {
    println!("Guessing number");

    let secert_num = rand::thread_rng().gen_range(1, 101);
    // 随机数生成器::随机数生成函数

    println!("secert num is {}", secert_num);

    loop{
        let mut guess = String::new();  // 默认变量不可变
        // 加上 mut 可变
        // 类型推断 + 强类型
        // String 默认UTF-8编码

        io::stdin().read_line(&mut guess).expect("无法读取");
        // 返回 io::Result （是一个enums类型，有两个 OK 与 Err
        // Ok 表示成功，包含返回值
        // Err 表失败，包含失败原因
        // except 对 Err 会中断，Ok 则返回结果

        // io 也可以换为 std::io
        // 这里有一个 & 取地址，也需要 mut 标记为可变
        // 这里读入的 guess 中包含换行

        // shadow
        // guess 又被声明了一次，同名新变量
        let guess: i32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue, // _ 表示不关心里头是什么
        };      // 用 match 处理出错
        // trim 去掉两端空白（空格、换行。。。）
        // parse 变数字
        // 显示声明变量类型

        println!("guessing number is {}", guess);
        // 与 python 类似 {} 占位

        match guess.cmp(&secert_num){
            // match 根据返回类型决定做什么
            // 这里有三个 arm 
            // Ordering 枚举类型也就三个 Less Greater Equal
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Win!");
                break;
            },
        }
    }
}
