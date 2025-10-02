
# Rust 安装和版本切换

## Rust 安装 
[官方安装链接](https://rust-lang.org/tools/install/)  
rust 安装官方命令 `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` 中国大陆地区使用有问题，最好使用镜像源

### 配置国内源[字节源](https://rsproxy.cn/)
步骤一：设置 Rustup 镜像， 修改配置 ~/.zshrc or ~/.bashrc
```bash
export RUSTUP_DIST_SERVER="https://rsproxy.cn"
export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
```

步骤二：安装 Rust（请先完成步骤一的环境变量导入并 source rc 文件或重启终端生效）
`curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh`

步骤三：设置 crates.io 镜像， 修改配置 ~/.cargo/config，已支持git协议和sparse协议，>=1.68 版本建议使用 sparse-index，速度更快。
```toml
[source.crates-io]
replace-with = 'rsproxy-sparse'
[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"
[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"
[net]
git-fetch-with-cli = true
```

### rust 更新
```bash
rustup update
```

### Rust 常见命令

* 查看版本
```bash
$ rustc --version
rustc 1.90.0 (1159e78c4 2025-09-14)
```

* 添加组件
```bash
$ rustup component add rustfmt
```

* 打开官方文档
```bash
$ rustup doc
```

* 查看已安装的工具链
```bash
$ rustup show
```

## Rust 版本切换

### 稳定版 stable（生产环境使用）

* 发布周期：每六周。
* 特点：这是绝大多数用户和生产环境应该使用的版本。它经过了充分的测试，所有功能都是永久稳定的，向后兼容。
* 版本号：例如 1.78.0, 1.79.0。每次发布都会包含之前测试版和夜间版中已经稳定的新功能和改进。

### 测试版 beta

* 发布周期：每六周，在稳定版发布前。
* 特点：这是下一个即将发布的稳定版的候选版本。它主要用于测试，以确保下一个稳定版的质量。普通用户一般不需要使用。

### 夜间版 nightly

* 发布周期：每天。
* 特点：包含所有最新、最前沿的功能，其中一些功能可能还不稳定（需要特性标志才能开启）。这个版本主要供语言开发者、标准库开发者以及需要体验最新实验性功能的用户使用。

### 版本切换

Rust 官方提供了一个非常强大的工具链管理工具：rustup。当你安装 Rust 时，它通常会一并安装。通过 rustup，你可以轻松安装和切换不同版本的工具链。

#### 安装其他版本的工具链

你可以安装特定的稳定版、测试版或夜间版。
```bash
# 安装最新的稳定版
rustup update stable

# 安装特定的稳定版（例如 1.75.0）
rustup install 1.75.0

# 安装夜间版
rustup install nightly

# 安装测试版
rustup install beta
```

#### 切换全局默认版本
这是最常用的切换方式，它会改变你终端中任何地方使用的 rustc 和 cargo 命令的版本。

```bash
# 将默认工具链设置为稳定版
rustup default stable

# 将默认工具链设置为特定的稳定版（例如 1.75.0）
rustup default 1.75.0

# 将默认工具链设置为夜间版
rustup default nightly
执行后，你可以通过 rustc --version 来验证是否切换成功。
```

#### 在特定目录中覆盖版本（按项目切换）

如果你正在参与一个要求使用特定 Rust 版本的项目（比如需要夜间版来使用一些实验性功能），你可以在项目目录中设置一个“本地覆盖”，这样在这个目录下，使用的 Rust 版本会自动切换。

```bash
# 进入你的项目目录
cd /path/to/your/project

# 为此目录设置使用夜间版
rustup override set nightly

# 为此目录设置使用特定的稳定版
rustup override set 1.75.0
```
设置后，rustup 会在这个目录中创建一个 rust-toolchain 文件（或更新该文件）。强烈建议将此文件提交到版本控制（如 Git）中，这样所有克隆该项目的协作者都会自动使用正确的 Rust 版本。

#### 使用 rust-toolchain.toml 文件（推荐）
这是更现代、更声明式的方法。你可以在项目的根目录下创建一个名为 rust-toolchain.toml 的文件，并指定所需的工具链。

```toml
# rust-toolchain.toml
[toolchain]
channel = "nightly-2024-05-20" # 可以指定精确的日期，也可以直接用 "nightly", "stable", "beta"
# components = [ "rust-analyzer" ] # 还可以指定需要安装的额外组件
```
当你在该项目目录下运行 cargo 或 rustc 时，rustup 会自动读取这个文件并切换到指定的工具链。这是管理项目特定版本的最佳实践。

