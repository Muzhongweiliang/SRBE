fn main() {
    println!("1.2.2 显示 Display");
    println!("笔记路径./docs/01_hello_world/02_02_display.md");

    println!("--------------------------------------------------------");
    // 通过 `use` 导入 `fmt` 模块使其可用.
    use std::fmt;

    // 定义一个结构体,我们将为其实现 `fmt::Display`.
    // 这是一个名为 `Structure` 的元组结构体,包含一个 `i32`.
    struct Structure(i32);

    // 要使用 `{}` 标记,必须为该类型手动实现 `fmt::Display` trait.
    impl fmt::Display for Structure {
        // 可读性更好的写法,模块路径完整且显示标出生命周期
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // 将第一个元素严格写入提供的输出流 `f`.
            // 返回 `fmt::Result`,表示操作是否成功.
            // 注意 `write!` 的语法与 `println!` 非常相似.
            write!(f, "{}", self.0)
        }
    }

    println!("Display Structure: {}", Structure(666));

    println!("--------------------------------------------------------");
    // 定义一个包含两个数字的结构体.
    // 派生 `Debug` 特性,以便与 `Display` 的结果进行对比.
    #[derive(Debug)]
    #[allow(dead_code)]
    struct MinMax(i64, i64);

    // 为 `MinMax` 实现 `Display` 特性.
    impl fmt::Display for MinMax {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // 使用 `self.number` 引用每个位置的数据点.
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    // 定义一个结构体,其字段可命名以便和 `MinMax` 比较.
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Point2D {
        x: f64,
        y: f64,
    }

    // 同样,为 `Point2D` 实现 `Display` 特性.
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // 自定义实现,只显示 `x` 和 `y`.
            write!(f, "x:{}, y:{}", self.x, self.y)
        }
    }

    let minmax = MinMax(0, 14);

    println!("比较结构体:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!(
        "大范围是 {big}, 小范围是 {small}",
        small = small_range,
        big = big_range
    );

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("比较点:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // 错误. 虽然实现了 `Debug` 和 `Display`,但是 `{:b}`需要实现 `fmt:Binary`.
    // 下面这行代码无法工作.
    // println!("Point2D 的二进制表示是什么: {:b}?", point);
}
