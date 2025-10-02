# 向量

写法
Vec `<T>`, 叫做 vector（向量，动态数组）
由标准库提供
可以存储多个值
只能存储相同类型数据
值在内存中连续存放

## 常见方法
方法|	签名 |	说明
--|--|--
new()	|pub fn new()->Vec	|创建一个空的向量的实例
push() |	pub fn push(&mut self, value: T)	| 将某个值 T 添加到向量的末尾
pop()	| pub fn pop(&mut self) -> Option `<T>` |	弹出最后一个元素
remove() |	pub fn remove(&mut self, index: usize) -> T |	删除并返回指定的下标元素。
contains() |	pub fn contains(&self, x: &T) -> bool |	判断向量是否包含某个值
len()	| pub fn len(&self) -> usize |	返回向量中的元素个数
swap()|	pub fn swap(&mut self, a: usize, b: usize) |	元素位置交换，a，b 向量下标
reverse()|	pub fn reverse(&mut self) |	反转元素

### 创建  vec
`Vec::new()`

例子
```rust
fn main(){
    let mut v: Vec<i32> = Vec::new();
    let v1 = vec![1,3,4,5];
    println!("{:?}",v);
    println!("{:?}",v1);
}
// 结果
// []
// [1, 3, 4, 5]
```

### 更新 vector

使用 push 方法

例子
```rust
let mut v2 = Vec::new();
v2.push(1);
v2.push(2);
v2.push(3);
println!("{:?}",v2);
// 结果
// [1,2,3]
```

### 删除 vector
与其他结构体一样 当 vector 离开作用域后,就会被清理掉,所有元素也被清理掉。

```rust
fn main(){
  {
    let arr2 = vec![4,5,6,2];
  }
  println!("{}",arr2);
}
// the binding `arr2` is available in a different scope in the same function
```

### 读取 vector
方式1. 索引下标，超出索引程序会恐慌 panic

```rust
let v3 = vec![31,46,52,61,2];
println!("vec[3] is: {}", &v3[3]);
// 结果
// vec[3] is: 61
```

方式2. get方法 返回 Option `<T>` 类型需要使用 match，超出索引没有执行 if let

```rust
let v3 = vec![31,46,52,61,2];
if let Some(x) = v3.get(3) {
    println!("vec.get[3] is: {}", x);
}
// 结果
// vec.get[3] is: 61
```

### 所用权规则
例子

```rust
let mut v4 = vec![2.4, 2.0, 23.0];
let v5 = &v4;  //不可变借用
v4.push(8.0);  //可变借用
println!("{:?}", v5);  // 不可变借用

// 报错
error[E0502]: cannot borrow `v4` as mutable because it is also borrowed as immutable
```

向量 == 动态数组 （可变的）存储在堆山

### 遍历向量

#### 方式1
```rust
let arr = vec![2,3,4,5];
for x in &arr{  // 一般使用 &arr 借用vec，不直接用vec
  println!("{}",x);
}
```

#### 方式2
```rust
for x in 0..arr.len() {
  println!("{}",arr[x]);
}
```

#### 方式3
```rust
for x in 0..4 {
  println!("{}",arr[x]);
}
```

#### 方式4 使用迭代器
```rust
for x in arr.iter(){
  println!("{}",x);
}
```

### 方式5， 可变引用并修改值
```rust
let mut v6 = vec![4,6,7,4,2];
for i in &mut v6{
    *i += 1;
}
println!("{:?}", v6);
// 结果
[5, 7, 8, 5, 3]
```

vector + enum
使用 enum 来存储多种数据类型
例子

```rust
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main(){
    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(1.0),
        SpreadsheetCell::Text(String::from("hello")),
    ];
    println!("{:?}", row);
}
// 结果
[Int(1), Float(1.0), Text("hello")]
局限性：
通过枚举需要知道 vector 的数据类型
```