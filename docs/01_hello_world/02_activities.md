## 1.2 格式化打印
### 练习

```Rust
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

```

- 修复上述代码中的问题(参见 FIXME 注释),使其能够正常运行.
- 尝试取消注释那行尝试格式化`Structure`结构体的代码(参见 TODO 注释)
- 添加一个`println!`宏调用,打印:`Pi`约等于`3.142`,通过控制显示的小数位数来实现.在本练习中,使用`let pi = 3.141592`作为`pi`的近似值.<br>
提示:你可能需要查阅`std::fmt`文档来了解如何设置显示的小数位数.