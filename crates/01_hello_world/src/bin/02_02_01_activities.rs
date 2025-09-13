fn main() {
    println!("1.2.2.1 练习");
    println!("笔记路径./docs/01_hello_world/02_02_01_activities.md");

    println!("--------------------------------------------------------");
    use std::fmt;

    #[allow(dead_code)]
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let vec = &self.0;

            write!(f, "[")?;

            for (index, v) in vec.iter().enumerate() {
                if index != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", index, v)?;
            }

            write!(f, "]")
        }
    }

    let list = List(vec![1, 2, 3]);
    println!("{}", list);
}
