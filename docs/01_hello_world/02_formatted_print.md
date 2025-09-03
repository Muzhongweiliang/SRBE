## 1.2 格式化打印

Rust中`std::fmt`定义了一系列`宏`处理打印功能<br>
其中包括:
- `format!`: 将格式化文本写入`String`
- `print!`: 与`format!`类似,但文本会打印到控制台(io::stout) 
- `println!`: 与`print!`类似,但会在末尾添加换行符
- `eprint!`:与`print!`类似,但文本会打印到标准错误输出(io::stderr)
- `eprintln!`: 与`eprint!`类似,但会在末尾添加换行符

所有这些宏都以相同的方式解析文本.<br>
此外,Rust会在编译时检查格式化的正确性.

```Rust
fn main() {
    // 通常，`{}` 会被自动替换为任何参数。
    // 这些参数会被转换为字符串。
    println!("{} 天", 31);

    // 可以使用位置参数。在 `{}` 中指定一个整数
    // 来决定替换哪个额外的参数。参数编号
    // 从格式字符串后立即开始，从 0 开始。
    println!("{0}，这是 {1}。{1}，这是 {0}", "Alice", "Bob");

    // 还可以使用命名参数。
    println!("{subject} {verb} {object}",
             object="那只懒惰的狗",
             subject="那只敏捷的棕色狐狸",
             verb="跳过");

    // 在 `:` 后指定格式字符，
    // 可以调用不同的格式化方式。
    println!("十进制：               {}",   69420); // 69420
    println!("二进制：               {:b}", 69420); // 10000111100101100
    println!("八进制：               {:o}", 69420); // 207454
    println!("十六进制：             {:x}", 69420); // 10f2c

    // 可以指定宽度来右对齐文本。这将输出
    // "    1"。（四个空格和一个 "1"，总宽度为 5。）
    println!("{number:>5}", number=1);

    // 可以用额外的零来填充数字，
    println!("{number:0>5}", number=1); // 00001
    // 通过翻转符号来左对齐。这将输出 "10000"。
    println!("{number:0<5}", number=1); // 10000

    // 在格式说明符后添加 `$` 可以使用命名参数。
    println!("{number:0>width$}", number=1, width=5);

    // Rust 甚至会检查使用的参数数量是否正确。
    println!("我的名字是 {0}，{1} {0}", "Bond");
    // FIXME ^ 添加缺失的参数："James"

    // 只有实现了 fmt::Display 的类型才能用 `{}` 格式化。
    // 用户定义的类型默认不实现 fmt::Display。

    #[allow(dead_code)] // 禁用 `dead_code`，它会警告未使用的模块
    struct Structure(i32);

    // 这无法编译，因为 `Structure` 没有实现 fmt::Display。
    // println!("这个结构体 `{}` 无法打印...", Structure(3));
    // TODO ^ 尝试取消注释这一行

    // 在 Rust 1.58 及以上版本，你可以直接从周围的变量捕获参数。
    // 就像上面一样，这将输出 "    1"，4 个空格和一个 "1"。
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
```
`std::fmt`包含许多控制文本显示的 traits
下面列出了两个重要的基本形式:

`fmt::Debug`:使用`{:?}`标记,用于调试目的的文本格式化.
`fmt::Display`:使用 `{}`标记.以更优雅且用户友好的方式格式化文本.<br>
这里我们使用`fmt::Display`,因为标准库为这些类型提供了实现.<br>
要打印自定义类型的文本,需要额外的步骤.

实现`fmt::Display`特性会自动实现 `ToString`特性,这允许我们将该类型转换为`String`.

在 43 行,`#[allow(dead_code)]`是一个属性(attribute),它只适用于它之后的模块.