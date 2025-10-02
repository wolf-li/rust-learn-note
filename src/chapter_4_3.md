# 枚举

enumerations 也称 enums，枚举允许列举可能的成员来定义一个类型。
在日常生活中很常见。比如：1 年有 12 个月，1 周有 7 天。
可以通过在代码中定义一个 IpAddrKind 枚举来表现这个概念并列出可能的 IP 地址类型，V4 和 V6。这被称为枚举的 成员（variants）：
enum IpAddrkind {
  v4, //自定义数据类型
  v6,
}
调用枚举
let ipv4 = IpAddrkind::v4;
let ipv6 = IpAddrkind::v6;
注意枚举的成员位于其标识符的命名空间中，并使用两个冒号分开。这么设计的益处是现在 IpAddrKind::V4 和 IpAddrKind::V6 都是 IpAddrKind 类型的。例如，接着可以定义一个函数来获取任何 IpAddrKind：
fn route(ip_kind: IpAddrKind) {}
现在可以使用任一成员来调用这个函数：
route(IpAddrKind::V4);
route(IpAddrKind::V6);
枚举保存类型，结构体保存 ip 类型和 ip 内容
enum IpAddrkind {
  v4,  // 没有关联任何数据 自定义数据类型
  v6,  // 没有关联任何数据 自定义数据类型
}

struct IpAddr {
  kind: IpAddrkind,
  address: String,
}

let home = IpAddr {
  kind: IpAddrkind::v4,
  address: String::from("127.0.0.1"),
}

let loopback = IpAddr {
  kind: IpAddrkind::v6,
  address: String::from("::0"),
}
优化上面代码
enum IpAddrkind {
  v4(String),
  v6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::0"));
标准库中 IpAddr 定义
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
枚举内嵌多种类型
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
这个枚举有四个含有不同类型的成员：
Quit 没有关联任何数据。
Move 类似结构体包含命名字段。
Write 包含单独一个 String。
ChangeColor 包含三个 i32。
定义一个如示例 6-2 中所示那样的有关联值的枚举的方式和定义多个不同类型的结构体的方式很相像，除了枚举不使用 struct 关键字以及其所有成员都被组合在一起位于 Message 类型下。如下这些结构体可以包含与之前枚举成员中相同的数据：
struct QuitMessage; // 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体
结构体和枚举还有另一个相似点：就像可以使用 impl 来为结构体定义方法那样，也可以在枚举上定义方法。这是一个定义于我们 Message 枚举上的叫做 call 的方法：
impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

let m = Message::Write(String::from("hello"));
m.call();
Option 枚举
rust 没有 Null
定义于标注库中
在 Prelude （预导入模块）中
描述了： 某个值可能存在（某种类型存在）或不存在的情况
在其他语言中：
null 是一个值，它表示‘没有值’
一个变量可以有两种状态： 空值 null ，非空
null 问题：
当尝试使用非 null 值那样使用 null 时，就会出现错误。
null 概念还是有用的：因某种原因变为无效或缺失值
match
控制流运算符
允许一个值与一系列模式进行匹配，并执行匹配的模式对应的代码
模式可以是字面值、变量名、通配符...
例子
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_coin(coin:Coin) -> i32 {
    match coin {
        // 匹配模式  => 返回结果
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main(){
    let one = Coin::Penny;
    println!("coin value is : {}",value_in_coin(one));
}

// 结果
coin value is : 1
绑定值模式
匹配的分支可以绑定到被匹配对象的部分值,可以从 enum 变体中提取值
例子
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_coin(coin:Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quater from {:?}", state);
            25
        },
    }
}

fn main(){
    let quarter = Coin::Quarter(UsState::Alabama);
    println!("coin value is : {}",value_in_coin(quarter));
}

// 结果
state quater from Alabama
coin value is : 25
匹配 Option `<T>`
例子
fn main(){
    let five = 5;
    let some_five = Some(5);
    let six = plus_one(some_five);
    println!("{:?}",six);
}

fn plus_one(num: Option `<i32>`) -> Option `<i32>`{
    match num {
        None => None,
        Some(i) => Some(i + 1),
    }
}
使用 match 必须要穷举所有可能
例子
fn plus_one(num: Option `<i32>`) -> Option `<i32>`{
    match num {
        Some(i) => Some(i + 1),
    }
}
// 报错
error[E0004]: non-exhaustive patterns: `None` not covered
   --> src/main.rs:3:15
    |
3   |         match x {
    |               ^ pattern `None` not covered
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `Option<i32>`

_ 通配符：替代其他没有列出的可能
例子
fn main(){
    let v = 1;
    match v {
        1 => println!("one");
        2 => println!("two");
        3 => println!("three");
        4 => println!("four");
        _ => (),
    }
}
if let 简单控制流
if 和 let 结合，使用简单方法匹配一种模式而忽略其他模式情况
例子
// 没有使用 if let
let config_max = Some(3u8);
match config_max {
  Some(max) => println!("the max config is: {}",max),
  _=> (),
}

// 使用 if let
let config_max = Some(3u8);
if let Some(max) = config_max {
   println!("the max config is: {}",max);
}

不使用 match 获取 Option<&str> 内值 (unwrap_or_default 方法)
let my_option: Option<&str> = Some("Hello");
let my_string: &str = my_option.unwrap_or_default();
println!("{}", my_string); // 输出: Hello
