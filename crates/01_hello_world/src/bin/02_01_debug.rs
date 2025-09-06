fn main() {
    println!("1.2.1 调试 Debug");
    println!("笔记路径./docs/01_hello_world/02_01_debug.md");

    println!("--------------------------------------------------------");
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Structure(i32);

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Deep(Structure);

    println!("现在 {:?} 将会打印!", Structure(3));

    println!("现在 {:?} 将会打印!", Deep(Structure(7)));
    // 只打印Deep(Structure(7))中的数字
    let d = Deep(Structure(7));
    println!("现在Deep(Structure(7))只打印数字 {} ", d.0.0);

    println!("--------------------------------------------------------");
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // 使用`{:#?}`美化打印
    println!("{:#?}", peter);
}
