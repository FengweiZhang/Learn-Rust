## basic

使用 rustc 编译 .rs 后缀文件

- 在windows 中生成 .exe .pdb(调试文件)

`fn` 定义函数

`*!` 表示 rust macro （rust 宏）（与函数不同）

tab 是4个空格


## cargo

`cargo new <name>`

- 生成 src

    - rs 源文件

- Cargo.toml

    - Tom's Obvious, Minimal Language

    - 是 cargo 的配置格式

    - [package] 配置包

    - [dependencies] 依赖项（在 rust 中，代码包被称为 crate）

- 顶层目录放源码无关文件

`cargo build`

- 自动生成 target 文件夹

    - debug 目录下放可执行文件

- 自动生成 Cargo.lock 文件夹

    - 第一次 build 时生成

    - 用于精确版本控制，不手动修改

`cargo run`

- build 并 执行（源码未更改则直接执行）

`cargo check`

- 检查

`cargo build --release`

- 会进行优化，时间长

- 位于 target/release



## guess num
