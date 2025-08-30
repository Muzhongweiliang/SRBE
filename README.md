# 1. 项目结构搭建
## 1.1 创建空仓库
```shell
cargo new srbe-cn --name srbe_root
cd srbe-cn
```

## 1.2 构建工作区
1. 删除项目根路径下的`src`文件夹
2. 替换项目根路径下的`Cargo.toml`代码,内容如下:
```toml
[workspace]  
resolver = "2"         # 依赖解析器v2 适用>=1.51 (2021及以后) 
members = ["crates/*"] # 把crates目录下的每一个子目录都当作workspace members   
```

## 1.3 章节<-->子crates生成
```shell
cargo new --bin crates\01_hello_world --name srbe_01_hello_world --vcs none
```
注意: 
1. 可执行文件使用`--bin` 参数,开发库使用`--lib`
2. 生成的`01_hello_world`路径名可以数字开头
3. 包名不能以数字开头,使用`--name`参数指定包名
4. 章节中`Cargo.toml`文件添加==publish = false==,工作区内部包,无需公开发布
5. `--vcs none`参数可以不为crate创建git仓库,可以被外面工作区当作普通文件进行版本管理

## 1.4 小节<-->crate下src/bin/
```
# 示意结构

crates/
└── 05_lifetimes/                             # 第5章 生命周期
    ├── Cargo.toml              
    └── src/
        ├── bin/                              # 每一小节可执行文件位置
        │   ├── 01_explicit.rs          
        │   ├── 02_elision.rs
        │   ├── 03_static.rs
        │   └── 04_trait_objects.rs
        └── lib.rs                            # 可选:公共辅助函数可放这里
```

## 1.5 小节中子节组织形式
```
crates/
└── 01_hello_world/                          # 第1章 你好世界
    ├── Cargo.toml              
    └── src/
        ├── bin                              # 每一小节及其子节可执行文件位置
        │   ├── 00_hello_world.rs            # 1.0
        │   ├── 01_comments.rs               # 1.1
        │   ├── 02_formatted_print.rs        # 1.2
        │   ├── 02_01_debug.rs               # 1.2.1
        │   ├── 02_02_display.rs             # 1.2.2
        │   ├── 02_02_01_testcae_list.rs     # 1.2.2.1
        │   └── 02_03_formatting.rs          # 1.2.3
        └── lib.rs                           # 可选:公共辅助函数可放这里
```

## 1.6 测试指令
```shell
 cargo run -p srbe_01_hello_world --bin 00_hello_world
```