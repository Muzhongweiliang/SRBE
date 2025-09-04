fn main() {
    println!("1.2 练习");
    println!("笔记路径./docs/01_hello_world/02_activities.md");

    println!("--------------------------------------------------------");
    println!("我的名字是 {0},{1} {0}", "weiliang", "Muzhong");

    println!("--------------------------------------------------------");
    // 只有实现了 fmt::Display 的类型才能用 `{}` 格式化。
    // 用户定义的类型默认不实现 fmt::Display。

    #[allow(dead_code)] // 禁用 `dead_code`，它会警告未使用的模块
    struct Structure(i32);

    // 这无法编译，因为 `Structure` 没有实现 fmt::Display。
    // println!("这个结构体 `{}` 无法打印...", Structure(3));
    // TODO ^ 尝试取消注释这一行

    /*
       error[E0277]: `Structure` doesn't implement `std::fmt::Display`
       --> crates\01_hello_world\src\bin\02_activities.rs:16:36
        |
     16 |     println!("这个结构体 `{}` 无法打印...", Structure(3));
        |                                             ^^^^^^^^^^^^ `Structure` cannot be formatted with the default formatter
    */

    println!("--------------------------------------------------------");
    let pi = 3.141592;
    println!("Pi 约等于 {:.3}", pi);
}
