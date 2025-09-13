use core::fmt;

fn main() {
    println!("1.2.2 练习");
    println!("笔记路径./docs/01_hello_world/02_02_activities.md");

    println!("--------------------------------------------------------");

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Complex {
        real: f64,
        imag: f64,
    }

    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} + {}i", self.real, self.imag)
        }
    }

    let complex = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);
}
