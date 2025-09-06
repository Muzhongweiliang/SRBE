## 1.2.1 调试 Debug
所有想要使用`std::fmt`格式化`traits`的类型都需要实现才能打印.自动实现仅为`std`库中的类型提供.所有其他类型都必须以某种方式手动实现.<br>
`fmt::Debug` trait 使这变得非常简单.所有类型都可以`derive`(自动创建)`fmt::Debug`实现.但这对`fmt::Display`不适用,后者必须手动实现.

```Rust
// 这个结构体无法通过 `fmt::Display` 或 `fmt::Debug` 打印。
struct UnPrintable(i32);

// `derive` 属性自动创建使这个 `struct` 可以用 `fmt::Debug` 打印的实现。
#[derive(Debug)]
struct DebugPrintable(i32);
```

所有`std`库类型也可以自动使用`{:?}`打印:

```Rust
// 为 `Structure` 派生 `fmt::Debug` 实现。`Structure` 是一个包含单个 `i32` 的结构体。
#[derive(Debug)]
struct Structure(i32);

// 在 `Deep` 结构体中放入一个 `Structure`。使其也可打印。
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // 使用 `{:?}` 打印类似于使用 `{}`。
    println!("{:?} 个月在一年中。", 12);
    println!("{1:?} {0:?} 是这个 {actor:?} 的名字。",
             "Slater",
             "Christian",
             actor="演员");

    // `Structure` 现在可以打印了！
    println!("现在 {:?} 将会打印！", Structure(3));

    // `derive` 的问题是无法控制输出的样式。
    // 如果我只想显示一个 `7` 怎么办？
    println!("现在 {:?} 将会打印！", Deep(Structure(7)));
}
```

所以`fmt::Debug`确实使其可打印,但牺牲了一些优雅. Rust 还提供了使用`{:#?}`进行"美化打印"的功能.

```Rust
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 美化打印
    println!("{:#?}", peter);
}
```

可以手动实现`fmt::Display`来控制显示方式.