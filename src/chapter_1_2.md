# 开发环境

## Rust 语言的编译器 rustc

* 查看版本

```bash
$ rustc -V
rustc 1.90.0 (1159e78c4 2025-09-14)
$ rustc --version
rustc 1.90.0 (1159e78c4 2025-09-14)
```

* 编译生成二进制文件

```bash
$ rustc -o output_filename filename.rs
```

* 编译生成库文件

```bash
$ rustc --crate-type lib filename.rs
```

## 开发环境搭建

必要安装内容

* gcc 
    * MacOS `brew install gcc` 包管理工具直接下载
    * Linux `apt install gcc`
* git [官方网站](https://git-scm.com/!)

不同操作系统

* Windows 系统
  WSL2（windows linux子系统）Ubuntu （Linux 发行版）   
  VScode 编辑器  
  stable-x86_64-pc-windows-msvc 工具链需要安装 VStudio2022 等。  
  stable-x86_64-pc-windows-gnu （不推荐使用）  
* Linux 发行版选用有图形界面的发行版如：Ubuntu
* MacOS 系统
  VScode 编辑器

公共依赖

VScode 插件

| 插件名称      | 作用 |
| ------------- | ---- |
| rust-analyzer |      |
| Error Lens    |       |
| Thunder Client | 轻量级的REST API客户端，可直接在VSCode内调试接口。  |
| Prettier      | 自动格式化代码，统一团队代码风格 |

