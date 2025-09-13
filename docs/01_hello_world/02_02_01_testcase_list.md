## 1.2.2.1 测试示例: 列表

为一个需要顺序处理元素的结构体实现 `fmt::Display` 是棘手的. 问题在于每个 `write!` 都会生成一个 `fmt::Result`. 正确处理这种情况需要处理所有的结果. Rust提供了 `?` 运算符专门用于此目的.

在 `write!` 上使用`?`的示例如下:

```Rust
// 尝试执行 `write!`, 检查是否出错. 如果出错,返回错误.
// 否则继续执行.
write!(f, "{}", value)?;
```

有了 `?` 运算符, 为了 `Vec` 实现 `fmt::Display` 就变得简单明了:

```Rust
use std::fmt; // 导入 `fmt` 模块。

// 定义一个名为 `List` 的结构体，包含一个 `Vec`。
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用元组索引提取值，
        // 并创建一个指向 `vec` 的引用。
        let vec = &self.0;

        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // index in `index`.
        for (index, v) in vec.iter().enumerate() {
            // 除第一个元素外，为每个元素添加逗号。
            // 使用 ? 运算符在出错时返回。
            if index != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // 闭合左括号并返回 fmt::Result 值
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
```