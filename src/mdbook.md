# mdbook


mdBook 是一个用于使用 Markdown 创建书籍的命令行工具。它非常适合用于创建产品或 API 文档、教程、课程材料或任何需要清晰、易于导航且可自定义呈现的内容。

[官方文档](https://rust-lang.github.io/mdBook/index.html)  

* 安装
`cargo install mdbook`  
!!! 注意 rust 版本，有时可能需要最新版才可以


* 卸载
`cargo uninstall mdbook.`

* 创建一本书
```bash
# 初始化
$ mdbook init my-first-book
# 进入目录
$ cd my-first-book
# 启动本地服务
$ mdbook serve --open
```
