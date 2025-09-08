## 显示 Display
`fmt::Debug`的输出往往不够简洁清晰,因此自定义输出外观通常更有优势.这可以通过手动实现`fmt::Display`来完成,它使用`{}`打印标记.实现方式如下:

```Rust
// 通过 `use` 导入 `fmt` 模块使其可用。
use std::fmt;

// 定义一个结构体，我们将为其实现 `fmt::Display`。
// 这是一个名为 `Structure` 的元组结构体，包含一个 `i32`。
struct Structure(i32);

// 要使用 `{}` 标记，必须为该类型手动实现 `fmt::Display` trait。
impl fmt::Display for Structure {
    // 这个 trait 要求 `fmt` 方法具有确切的签名。
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 将第一个元素严格写入提供的输出流 `f`。
        // 返回 `fmt::Result`，表示操作是否成功。
        // 注意 `write!` 的语法与 `println!` 非常相似。
        write!(f, "{}", self.0)
    }
}
```

`fmt::Display`可能比`fmt::Debug`更简洁,但这给`std`库带来了一个问题:如何显示歧义类型?例如,如果`std`库为所有`Vec<T>`实现统一的样式,应该采用哪种样式?是以下两种之一吗?

- `Vec<path>`: `/:/etc:/home/username:/bin`(以`:`分隔)
- `Vec<number>`: `1,2,3`(以`,`分隔)

答案是否定的.因为不存在适用于所有类型的理想样式,`std`库也不应擅自规定一种.因此,`fmt::Display`并未为`Vec<T>`或其他泛型容器实现.在这些泛型情况下,必须使用 `fmt::Debug`.

不过,这不是问题.对于任何新的非泛型容器类型,都可以实现`fmt::Display`.

```Rust
use std::fmt; // 导入 `fmt`

// 定义一个包含两个数字的结构体。派生 `Debug` 特性，
// 以便与 `Display` 的结果进行对比。
#[derive(Debug)]
struct MinMax(i64, i64);

// 为 `MinMax` 实现 `Display` 特性。
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用 `self.number` 引用每个位置的数据点。
        write!(f, "({}, {})", self.0, self.1)
    }
}

// 定义一个结构体，其字段可命名以便比较。
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// 同样，为 `Point2D` 实现 `Display` 特性。
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 自定义实现，只显示 `x` 和 `y`。
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("比较结构体：");
    println!("Display：{}", minmax);
    println!("Debug：{:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("大范围是 {big}，小范围是 {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("比较点：");
    println!("Display：{}", point);
    println!("Debug：{:?}", point);

    // 错误。虽然实现了 `Debug` 和 `Display`，但 `{:b}` 需要
    // 实现 `fmt::Binary`。这行代码无法工作。
    // println!("Point2D 的二进制表示是什么：{:b}？", point);
}
```

因此,虽然实现了`fmt::Display`,但未实现 `fmt::Binary`,所以无法使用.`std::fmt`包含许多这样的traits,每个都需要单独实现.更多详情请参阅[std::fmt](https://doc.rust-lang.org/std/fmt/).
