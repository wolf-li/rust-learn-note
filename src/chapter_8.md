# 泛型

rust 泛型
泛型： 可以在运行时指定数据类型的机制。
泛型好处：一套代码可以应用多种类型。比如向量可以是整型，也可以是浮点型。
泛型 既能保证代码安全又可以减少代码量，现代语言，没有泛型简直就是鸡肋。
Rust 语言中泛型主要包含 泛型集合、泛型结构体、泛型函数、泛型枚举和特质几个方面。
Rust 语言中的泛型
Rust 语言使用 <T> 语法来实现泛型指定类型，其中 T 可以是任意数据类型。
例如下面的两个类型 
age: i32
age: String
用泛型声明
age: T
泛型集合
常见的泛型集合 (collection)，例如 Vec 向量，它可以是一个字符串向量，也可以是一个整型向量。
声明一个整型向量
fn main(){
    let mut vec_integer :Vec<i32> = vec![32,40];
    vec_integer.push(79);
    println!("{:?}", vec_integer);
}
结果
[32, 40, 79]
向量定义类型后向量内的元素必须是相同类型的
fn main(){
    let mut vec_integer :Vec<i32> = vec![32,40];
    vec_integer.push(79);
    vec_integer.push("string");  // 报错
    println!("{:?}", vec_integer);
}

// 报错信息
error[E0308]: mismatched types
 --> vec.rs:4:22
  |
4 |     vec_integer.push("string");
  |                 ---- ^^^^^^^^ expected `i32`, found `&str`
  |                 |
  |                 arguments to this method are incorrect
  |
note: method defined here
 --> /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/alloc/src/vec/mod.rs:1967:12

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0308`.
泛型结构体
可以把某个结构体声明为泛型的，泛型结构体的成员类型也可以是泛型
定义语法
struct struct_name<T>{
  filed:T,
}
示例：
struct Data<T>{
  value:T,
}
例子：定义泛型结构体并使用
fn main(){
    let t:Data<i32> = Data{value:3};
    println!("{}", t.value);

    let t2:Data<String> = Data{value:"hello".to_string()};
    println!("{}", t2.value);
}

struct Data<T>{
    value: T,
}
结果
3
hello
特质 Traits
Rust 没有类、接口概念。
如果没有接口，如何证明两个结构体之间的关系呢？
Rust 提供了特质 Traits 这个蛋炒饭概念，用于跨多个结构体实现实现一组标准行为（方法）。
特质的定义语法
Rust 提供了关键字 trait 用于定义特质
特质就是一组方法的原型。语法格式如下
trait some_trait {
  // 没有任何实现的虚方法
  fn method1(&self);

  // 有具体实现的普通方法
  fn method2(&self){
     // 具体代码
  }
}
如果想让某个方法的定义在实现了特质的结构体所共享，那么推荐使用具体方法。
如果想让某个方法的定义由实现了特质的结构体自己定义，那么推荐使用抽象方法。
即使某个方法是具体方法，结构体也可以对该方法进行重写。



---

参考
https://www.twle.cn/c/yufei/rust/rust-basic-generic-types.html
