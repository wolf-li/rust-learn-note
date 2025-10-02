# Cargo 包管理工具

隐式使用 rustc 进行编译

## 常用命令

* 创建
    * 初始化 Rust 项目 `cargo new project_name`
    * 初始化 Rust库 项目 `cargo new --lib project_name`
* 构建项目（生成二进制可执行文件或库文件）
    * 编译调试版 `cargo build`
    * 编译可以发布的版本 `cargo build --release`
* 最快编译检查，不产生可执行文件
    `cargo check`
* 按官方风格格式化全部代码
    `cargo fmt`
* 编译+立即运行
    `cargo run`
* 跑单元+集成测试
    `cargo test`
* 整个工作区一次性全测
    `cargo test --workspace`
* 以树形打印依赖关系
    `cargo tree`
* 添加依赖库 (rust 1.62 版本以上)
    `cargo add dependency_name`
    指定版本 `cargo add dependency_name@0.1.1`
* 删除依赖库
    `cargo rm dependency_name`

## 项目结构

二进制项目结构 `cargo new project-test`

```text
project-test/  
├── Cargo.toml  
└── src  
    └── main.rs  
```

库项目结构 `cargo new --lib lib-test`

```text
lib-test/  
├── Cargo.toml  
└── src  
    └── lib.rs  
```

## 项目文件

Cargo.toml和Cargo.lock各有其目的。在我们谈论它们之前，这是一个总结:

Cargo.toml是从广义上描述你的依赖，并由你编写.
Cargo.lock包含有关您的依赖项的确切信息。它由 Cargo 维护，不应手动编辑.
如果您正在构建，其他项目要依赖的库，请将Cargo.lock放置在你的.gitignore。如果您正在构建可执行文件，如命令行工具或应用程序，请检查Cargo.lock位于git管理下。如果你对这是为什么感到好奇，请参阅"为什么二进制文件在版本控制系统中有Cargo.lock，而库没有?" .

让我们再挖掘一下.

Cargo.toml是一个manifest(清单)，我们可以在其中指定一系列关于我们项目的不同元数据的文件。例如，我们可以说我们依赖于另一个项目:

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git" }
```
这个项目有一个依赖关系rand箱。在这种情况下，我们已经说过，我们依赖于 GitHub 上的特定 Git 存储库。由于我们尚未指定任何其他信息，因此 Cargo 假定我们打算使用最新提交的master分支构建我们的项目。

听起来不错? 嗯，但有一个问题: 如果你今天构建这个项目，然后你发送一份副本给我，我明天构建这个项目，可能会发生一些不好的事情。因在此期间，可能会有更多的rand提交，我的构建将包括新的提交，而你的不会。因此，我们会得到不同的构建。这很糟糕，因为我们需要可重复的构建.

我们可以通过放置一个rev来解决这个问题，写入我们Cargo.toml:

```toml
[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git", rev = "9f35b8e" }
```
现在我们的构建将是相同的。但是有一个很大的缺点:现在我们每次想要更新库时，都必须手动考虑 SHA-1。这既乏味又容易出错.

那现在Cargo.lock登场了。由于它的存在，我们不需要手动跟踪确切的修订版本: Cargo 将为我们做。当我们有这样的清单时:

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]

[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand.git" }
```
Cargo 将采取最新的提交，并在我们第一次构建时，将这些信息写入我们的Cargo.lock。该文件将如下所示:

```toml
[root]
name = "hello_world"
version = "0.1.0"
dependencies = [
 "rand 0.1.0 (git+https://github.com/rust-lang-nursery/rand.git#9f35b8e439eeedd60b9414c58f389bdc6a3284f9)"，
]

[[package]]
name = "rand"
version = "0.1.0"
source = "git+https://github.com/rust-lang-nursery/rand.git#9f35b8e439eeedd60b9414c58f389bdc6a3284f9"
```
你可以看到这里有更多的信息，包括我们用来构建的确切修订版本。现在，当您将项目交给其他人时，他们将使用完全相同的 SHA，即使我们没有在我们的项目Cargo.toml中指定它.

当我们准备选择，更新库的版本时，Cargo 会自动重新计算依赖关系，并为我们更新内容:

```bash
$ cargo update           # updates all dependencies
$ cargo update -p rand   # updates just “rand”
```
这将写出一个新的Cargo.lock与新版本信息。请注意cargo update参数，实际上会是是一个包 ID 规范，和rand只是一个简短的规范.
