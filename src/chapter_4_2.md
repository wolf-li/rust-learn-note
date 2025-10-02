# String与&str

Rust 语言提供两种字符串

## String

* String 是一个堆分配的可变字符串类型  
Rust源码：  
```Rust
#[derive(PartialEq, PartialOrd, Eq, Ord)]
#[stable(feature = "rust1", since = "1.0.0")]
#[lang = "String"]
pub struct String {
    vec: Vec<u8>, // `Vec<u8>` Vec 可变数组，u8 无符号整数类型可以表示 0~255 之间的数
}
```

### 字符串如何存储   
例子：  
11100100 10111101 10100000 11100101 10100101 10111101  -> 你好
```python
text = "你好"
binary_data = text.encode('utf-8')  # 或者直接 text.encode()

print("原始字符串:", text)
print("UTF-8 编码的字节:", binary_data)
print("十六进制表示:", binary_data.hex())
print("二进制表示:", ' '.join(format(byte, '08b') for byte in binary_data))
print("字节数组:", list(binary_data))
```  

* 何时结束
    * 一种存储结束字符 '\0' C语言中使用
    * 一种在开始存储 len （长度） Rust 使用
* 如何编码
    * ASCII 一个字节对应一个字符（可以表示 0~255个字符）
    * UTF-8 一个到四个字节对应（可以辨识所有语言字符，除了私人语言，可以兼容 ASCII）


## &str

* &str 是指字符串切片引用（没有所有权），是在栈上分配的


字符串困扰 rust 开发者原因

rust 倾向于暴露可能的错误

字符串数据结构复杂

utf-8

Rust 的核心语言层面，只有一个字符串类型：字符串切片 str 或 &str

字符串切片：对于存储在其他地方、utf-8 编码的字符串的引用

    字符串字面值：存储在二进制文件中，也是字符串切片

String 类型：

来自标准库而不是 核心语言

可增长、可修改、可拥有

utf-8 编码

Rust 中通常说的 字符串是指： String str

其他类型的字符串

Rust 标准库还包含很多其它的字符串类型，例子：OsString、OsStr、CSting、 CStr

String vs str 后缀，拥有或借用的变体

可存储不同编码的文本或者在内存中以不同的形式展现

创建一个新的字符串

```rust
String::new() 函数

let mut s = String::new();
```

使用初始值来创建 String

`to_string()` 方法，这个方法已经实现了 Display trait ，包括字符串的字面值

```rust
let data = "hello";

let s = data.to_string();

let s1 = "hello".to_string();

String::from() 函数，从字面值创建 String

let s = String::from("hello");
```

utf-8 编码 例子

```rust
fn main() {

    let hello = String::from("السلام عليكم");

    println!("{}",hello);

    let hello = String::from("Dobrý den");

    println!("{}",hello);

    let hello = String::from("Hello");

    println!("{}",hello);

    let hello = String::from("שָׁלוֹם");

    println!("{}",hello);

    let hello = String::from("नमस्ते");

    println!("{}",hello);

    let hello = String::from("こんにちは");

    println!("{}",hello);

    let hello = String::from("안녕하세요");

    println!("{}",hello);

    let hello = String::from("你好");

    println!("{}",hello);

    let hello = String::from("Olá");

    println!("{}",hello);

    let hello = String::from("Здравствуйте");

    println!("{}",hello);

    let hello = String::from("Hola");

    println!("{}",hello);

}

// 结果

السلام عليكم

Dobrý den

Hello

שָׁלוֹם

नमस्ते

こんにちは

안녕하세요

你好

Olá

Здравствуйте

Hola
```

更新 String

`push_str()` 方法，字符串切片添加， 不会获得参数所有权

```rust
let mut s = "hello".to_string();

s.push_str(" word");

println!("{}", s);

// 结果
hello word
```

`push()` 方法，单个字符附加到 String

+ 拼接 String

```rust
let mut str2 = "你好".to_string();

str2.push('s');

println!("{}", str2);

// 结果

你好s
```

+ 拼接字符串

```rust
let str3 = "i ".to_string();

let str4 = " love ".to_string();

let str5 = str3 + &str4;  ( 格式固定 str + &str ,后面 str3 变量不能使用)

println!("{}", str5);

// 结果

i  love
```

format 宏拼接 String，不会取的字符串的所有权  推荐使用
```rust
let str6 = "你是".to_string();

let str7 = "谁".to_string();

let str8 = format!("{}{}",str6, str7);

println!("{}", str8);

// 结果

你是谁
```

String 按索引的方式访问

按照索引下标进行访问程序报错

```rust
let str1 = "whoareyou".to_string();

println!("{}",str1[0]);
```

String 内部表示

String 是对 Vec `<u8>` 的包装

方法  | 原型  | 说明
--|--|--
new()   |pub const fn new() -> String    | 创建一个新的字符串对象
to_string()| fn to_string(&self) -> String   | 将字符串字面量转换为字符串对象
replace()   |pub fn replace<'a, P>(&'a self, from: P, to: &str) -> String    | 搜索指定模式并替换
as_str()    | pub fn as_str(&self) -> &str    | 将字符串对象转换为字符串字面量
push()|  pub fn push(&mut self, ch: char)    | 再字符串末尾追加字符
push_str()|  pub fn push_str(&mut self, string: &str)    | 再字符串末尾追加字符串
len()   |pub fn len(&self) -> usize  | 返回字符串的字节长度
trim()  |pub fn trim(&self) -> &str  | 去除字符串首尾的空白符
split_whitespace()|  pub fn split_whitespace(&self) -> SplitWhitespace   | 根据空白符分割字符串并返回分割后的迭代器
split() |pub fn split<'a, P>(&'a self, pat: P) -> Split<'a, P>   根据指定模式分割字符串并返回分割后的迭代器。模式 P | 可以是字符串字面量或字符或一个返回分割符的闭包
chars() | pub fn chars(&self) -> Chars    | 返回字符串所有字符组成的迭代器


### `len()` 函数使用

```rust
let str1 = "世界和平".to_string();
println!("{}",str1.len());
// 结果
12
```

概念

字节、标量值、字形簇

bytes() 获得字符串对应字节, 字节就是计算机存储字符串样子

```rust
let str1 = "库里".to_string();
println!("{:?}",str1.bytes());

// 结果
Bytes(Copied { it: Iter([229, 186, 147, 233, 135, 140]) })
```

标量值 chars()

```rust
let str1 = "立棍单打".to_string();
println!("{:?}", str1.chars());

// 结果
Chars(['立', '棍', '单', '打'])
```

字符串切片操作（需要根据字符串占用字节数进行切割，操作需谨慎，切割过多或果断就会引起程序恐慌 panic）

```rust
let str1 = "立棍单打".to_string();

println!("{} {} {}", &str1[0..3], &str1[3..6] , &str1[0..6]);

// 结果
```

