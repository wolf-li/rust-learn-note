# HashSet 哈希

哈希集合 HashSet 简称集合，没有重复值的相同类型数据集合。
集合的最大特征就是没有重复值。
Rust 标准库 std::collections 中定义了 HashSet 描述集合

方法|	方法原型| 描述
--|--|--
insert()|	pub fn insert(&mut self, value: T) -> bool | 插入一个值到集合中如果集合已经存在值则插入失败
len()	| pub fn len(&self) -> usize	| 返回集合中的元素个数
get()	| pub fn get<Q:?Sized>(&self, value: &Q) -> Option<&T> |	根据指定的值获取集合中相应值的一个引用
iter()	| pub fn iter(&self) -> Iter | 返回集合中所有元素组成的无序迭代器迭代器元素的类型为 &'a T
contains_key |	pub fn contains<Q: ?Sized>(&self, value: &Q) -> bool |	判断集合是否包含指定的值
remove()	| pub fn remove<Q: ?Sized>(&mut self, value: &Q) -> bool	 |从结合中删除指定的值

### 创建集合
`new()`  HashSet 中的静态方法用于创建一个集合的实例。
`let mut set = HashSet::new();  // 下文没有声明或更改集合元素将会报错需要指定元素类型`

### 集合中插入值

`insert()` 插入一个值到已知集合中，如果集合内有该元素则插入失败
代码

```rust
fn main() {
    let mut set1 = HashSet::new();
    set1.insert("k");
    set1.insert("m");
    set1.insert("b");
    let x = set1.insert("k");
    println!("hashset set1 value: {:?}", set1);
    println!("result x: {}", x);
}
// 结果
hashset set1 value: {"b", "k", "m"}
result x: false
```

### 获取集合长度
len() 用于获取集合中元素的个数
代码
```rust
fn main() {
    let mut set1 = HashSet::new();
    set1.insert("k");
    set1.insert("m");
    set1.insert("b");
    set1.insert("k");
    println!("hashset set1 value: {:?}", set1);
    println!("hashset set1 length: {:?}", set1.len());
}
// 结果
hashset set1 value: {"b", "k", "m"}
hashset set1 length: 3
```

返回集合所有元素创建的迭代器 iter()
iter() 将集合中所有元素组成无序迭代器
迭代器内元素没有顺序，每次结果都可能不一样
代码
```rust
set1.insert("k");
set1.insert("m");
set1.insert("b");

println!("hashset set1 value: {:?}", set1);
println!("hashset set1 length: {:?}", set1.len());
for item in set1.iter(){
      println!("{}",item);
}
// 结果
hashset set1 value: {"m", "b", "k"}
hashset set1 length: 3
m
b
k

hashset set1 value: {"k", "m", "b"}
hashset set1 length: 3
k
m
b
```

### 获取集合中指定值的一个引用
get()  用于获取集合中指定值的引用
代码
```rust
let mut language = HashSet::new();
language.insert("python");
language.insert("php");
language.insert("javascript");
language.insert("golong");

match language.get(&"rust"){
    Some(value) => {
        println!("found: {}", value);
    }
    None => {
        println!("not found");
    }
}
// 结果
not found
```

### 判断集合中是否包含某个值
contains()  方法用于判断集合是否包含某个值
```rust
fn main() {
    let mut language = HashSet::new();
    language.insert("python");
    language.insert("php");
    language.insert("javascript");
    language.insert("golong");
    let judge = language.contains("rust");
    println!("language is include rust: {}", judge);
}
// 结构
language is include rust: false
```

### 删除集合元素
remove() 方法用于删除集合中指定元素

```rust
fn main() {
    let mut language = HashSet::new();
    language.insert("python");
    language.insert("php");
    language.insert("javascript");
    language.insert("golong");
    println!("language elements: {:?}, len {}", language, language.len());
    language.remove("golong");
    println!("language elements: {:?}, len {}", language, language.len());
}
// 结果
language elements: {"python", "javascript", "php", "golong"}, len 4
language elements: {"python", "javascript", "php"}, len 3
```