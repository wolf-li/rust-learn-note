# String与&str

Rust 语言提供两种字符串

## String

* String 是一个堆分配的可变字符串类型  
字符串对象 String。它不是 Rust 核心的一部分，只是 Rust 标准库中的一个 公开 pub 结构体。
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
    * 不可变引用，指向存储其他地方的 UTF-8 编码字符串数据
    * 由指针和长度构成
字符串字面量 &str。它是 Rust 核心内置的数据类型。

Rust 中通常说的 字符串是指： String str

其他类型的字符串

Rust 标准库还包含很多其它的字符串类型，例子：OsString、OsStr、CSting、 CStr

### &str 声明

字符串字面量 &str 就是在 编译时 就知道其值的字符串类型，是 Rust 语言核心的一部分。

字符串字面量 &str 是字符的集合，被硬编码赋值给一个变量。

&str 其他叫法： 字符串切片、字符串字面量

```rust
let name1: &str = "你好，Rust";  // 一般定义字符串字面量不写类型
// &str 常见声明方式
let name2 = "我是 Rust";
println!("{name1}");
println!("{name2}");
```

### 创建一个 String

* 创建一个新的字符串，使用 `String::new()` 静态方法
```rust
let mut s = String::new();
s = "this is a test".to_string();
println!("{s}");
```

* 根据字符串字面量创建，使用 `String::from()` 方法

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
```

### &str 转换为 String

`to_string()` 和 `to_owned()` 方法，这个方法已经实现了 Display trait ，包括字符串的字面值

```rust
let data = "hello s";
let s = data.to_string();
let s1 = "hello s1".to_owned();

println!("s: {s}");
println!("s1: {s1}");
```

### 更新 String

`push_str()` 方法，字符串切片添加， 不会获得参数所有权

```rust
let mut s = "hello".to_string();
s.push_str(" word");
println!("{}", s);
```

### `push()` 方法，单个字符附加到 String

```rust
let mut str2 = "你好".to_string();

str2.push('s');
println!("{}", str2);
```

### + 拼接字符串

```rust
let str3 = "i ".to_string();
let str4 = " love ".to_string();
let str5 = str3 + &str4;  ( 格式固定 str + &str ,后面 str3 变量不能使用)

println!("{}", str5);
```

### format 宏拼接 String，不会取的字符串的所有权  推荐使用

```rust
let str6 = "你是".to_string();
let str7 = "谁".to_string();
let str8 = format!("{}{}",str6, str7);

println!("{}", str8);
```

### String 按索引的方式访问

Rust 的 String 不允许直接按整数索引（如 s[i]）访问字符，UTF-8 编码下字节 ≠ 字符；随机字节切割可能落在字符中间，导致非法 UTF-8 片段，违反内存安全。

错误示例：  
```rust
let str1 = "whoareyou".to_string();
println!("{}",str1[0]);
```

按 字节进行切片，英文可以中文不行
```rust
let s = String::from("hello");
let sub = &s[1..4];   // &str 类型，字节范围 1..4
assert_eq!(sub, "ell");
```

```rust
let s = "中国人";
let _ = &s[1..2]; // panic: byte index 1 is not a char boundary
```

按「字符」随机访问 → 先收集成 Vec<char>
```rust
let s = "中国人";
let chars: Vec<char> = s.chars().collect();
println!("第三个字符：{}", chars[2]);
```


### String 常用方法

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


#### `len()` 函数使用

```rust
let str1 = "世界和平".to_string();
println!("{}",str1.len());

// 结果
// 12
```

### bytes() 获得字符串对应字节, 字节就是计算机存储字符串样子

```rust
let str1 = "库里".to_string();
println!("{:?}",str1.bytes());

// 结果
// Bytes(Copied { it: Iter([229, 186, 147, 233, 135, 140]) })
```

### 标量值 chars()

```rust
let str1 = "立棍单打".to_string();
println!("{:?}", str1.chars());

// 结果
// Chars(['立', '棍', '单', '打'])
```

## 字符串切片操作

>（需要根据字符串占用字节数进行切割，操作需谨慎，切割过多或果断就会引起程序恐慌 panic）

```rust
let str1 = "立棍单打".to_string();

println!("{} {} {}", &str1[0..3], &str1[3..6] , &str1[0..6]);
```

## String 与 &str 如何选择

注意 String 是有所有权的，而 &str 没有
* Struct 中属性使用 String
    * 如果不显示声明周期无法使用 &str
    * 使用麻烦还有隐患
* 函数参数推荐使用 &str （如果不想交出所有权）
    * &str 为参数，可以传递 &str 和 &String
    * &String 作为参数，只能传递 &String 不能传递 &str

