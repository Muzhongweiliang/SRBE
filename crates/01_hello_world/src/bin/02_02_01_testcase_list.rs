fn main() {
    println!("1.2.2.1 测试实例: 列表");
    println!("笔记路径./docs/01_hello_world/02_02_01_testcase_list.md");

    println!("--------------------------------------------------------");
    use std::fmt;

    // 定义一个名为 `List` 的结构体,包含一个 `Vec`.
    #[allow(dead_code)]
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // 使用元组索引提取值,
            // 并创建一个指向 `vec` 的引用.
            let vec = &self.0;

            write!(f, "[")?;

            // 在对遍历vec的过程中,同时获取每个元素的索引index和值v
            for (index, v) in vec.iter().enumerate() {
                // 除了第一个元素外,为每个元素添加逗号.
                // 使用 ? 运算符在出错时返回.
                if index != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", v)?;
            }

            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
