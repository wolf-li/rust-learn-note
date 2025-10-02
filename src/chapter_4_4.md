# Ownership

所有权
所有权核心特性
所有程序都必须管理它们使用计算机内存的形式
有的语言有垃圾回收机制，在程序运行时，它们会不断地寻找不再使用的内存 （java）
其他语言中程序员必须显示的分配和释放内存 （c c++）
rust 采用第三种方式进行内存管理
内存是通过所有权系统来管理的，其中包含一组编译器在编译时检查的规则
当程序运行时，所有权特性不会减慢程序运行速度
栈（stack）内存、堆（heap）内存
在像 rust 这样的系统级编程语言里，一个值是在 stack 还是 heap 上对于有的行为和为什么要做某些决定是有更大的影响的
在你的代码运行的时候， Stack 和 heap 都是你可以用的内存，但他们的结构很不相同
stack 、heap 存储数据
stack （栈）按值的接收顺序来进行存储，按相反的顺序将它们移除（后进先出）last in first out（栈不能从中间或底部抽取数据）增加数据叫入栈，移除数据叫出栈。
栈中所有数据占用必须是已知固定大小。（未知大小的数据或运行时可能发生变化的数据需要存储在 heap （堆）上）
Heap 内存组织性差一些
当数据放入 heap 时，会请求一定数据量的空间
操作系统在 heap 里找到一块足够大的空间，把它标记成再用，并返回一个指针，也就是这个空间的地址
这个过程叫在 heap 上进行分配，有时仅称为 分配
stack 存储与 heap 存储比较
入栈比在堆上分配内存要快
入栈时分配器无需为新数据搜索空间，位置总是在栈顶，在堆存储中分配器需要在内存中找到一块足够大的内存空间，并且需要为记录下一次数据做好准备 
访问堆上的数据比访问栈上的数据慢
heap 访问数据时通过指针进行访问，指针需要进行多次跳转，对于现代处理器来说，由于缓存的因素，指令在内存中跳转次数越少，速度越快
所有权解决的问题
跟踪代码在那些部分正在使用 heap 的那些数据
最小化 heap 上的重复数据量
清理 heap 上未使用的数据以避免空间不足
所有权规则
1.每个值都有一个变量，这个变量是所有者
2.每个值同时只能有一个所有者
3.当所有者超出作用域（scope）时，该值将被删除
英文原版
Each value in Rust has an owner.
There can only be one owner at a time.
When the owner goes out of scope, the value will be dropped.
变量作用域
作用域（scope）：程序中变量的有效范围
例子
{                    // s 在这里无效，没有声明  
  let s = 'hello';   // 从现在起 s 是有效的
  // 使用 s
}  // 此作用域已技术，s 不在有效
String 类型
String 比基础标量数据类型更复杂，演示所有权规则， 基本数据类型都存储在 stack 上，离开作用域后自动出栈，释放内存。
字符串字面值：程序里手写的字符串值，他们是不可变。是被硬编码进程序里的字符串值（类型为 &str ） let s ="hello"
字符串字面值不适合于所有场景：
字符串字面值不可变
并非所有字符串的值都能在编写代码时得知（如：用户输入数据）
Rust 提供动态字符串类型 String，该类型被分配到堆上，因此可以动态伸缩，可以存储编译时未知大小的文本
可以使用下面的方法创建基于字符串字面量来创建  string 类型：
let mut s = String::from("hello");  // 变量如果有变化需要声明为可变类型
s.push_str(",world");   // push_str() 在字符串后面添加 ',word'
println!("{}",s);       // 打印拼接后的字符串
内存和分配
字符串字面值在编译时候就知道其内容，直接被硬编码到最终程序中，速度块高效，正式因为其不可变形。
String 类型，为了支持可变行，需要在 heap 上分配来保存编译时未知的文本内容：
操作系统必须在运行时请求内存
当用完 String 类型后，需要以某种方式将内存归还给操作系统
在拥有 GC 的语言中，会追踪程序中不使用的内存并回收
没有 GC 的语言中，需要识别内存不使用后，返还给操作系统
如果忘记，浪费内存
如果提前释放，变量非法
如果重复释放，就是 Bug
rust 采用不同的方式，对于 rust 来说，变量走出作用域后，内存会立即交换给操作系统
变量和数据的交互方式
转移所有权
let x = 5;
let y = x;
代码逻辑 将 5 绑定给变量 x，接着拷贝 x 的值赋给 y ，最终 x 和 y 都等于 5，因为整数类型是基本类型，固定大小的简单之，这两个值都是通过自动拷贝方式来赋值的，都被存在栈中，不需要使用堆。
let s1 = String::from('hello');
let s2 = s1;
String 不是基本类型存储在堆上，不能自动拷贝。
String 类型，是由存储在栈中的 堆指针，字符串长度、字符串容量 共同构成。
堆指针（8字节）：指向真是存储字符串内容的堆内存
字符串容量（8字节）：是堆内存分配的空间大小
字符串长度（8字节）：目前已经使用的大小
根据所有权规则：一个值只允许有一个所有者，现在这个值有两个所有者，若变量离开作用域后， rust 会自动调用 drop 函数清理堆内存，不过由于两个String 变量指向了同一个问题，就会出现释放两次内存（二次释放错误），会造成内存污染，可能导致潜在安全问题。
Rust 是当 s1 赋予 s2后， rust 认为 s1 不在有效，因此无需在 s1 离开作用域后 drop 任何东西，直接把 所有权从 s1 转移给了 s2. s1在被赋予 s2 后马上失效的，这个操作成为移动
let s1 = String::from('hello');
let s2 = s1;
println!("{}",s1);
// 编译有问题
error[E0382]: borrow of moved value: `s1`  // 借用了已经移动的 s1 的值
 --> src/main.rs:4:16
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |     println!("{}",s1);
  |                   ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

For more information about this error, try `rustc --explain E0382`.
warning: `function` (bin "function") generated 1 warning
error: could not compile `function` due to previous error; 1 warning emitted
﻿﻿
克隆（深拷贝）
复制所有信息
rust 永远不会自动创建数据的深拷贝，因此任何自动复制都不是深拷贝，可以被认为对运行时影响较小，如果需要对 String 类似存储在 堆中的数据进行深拷贝，可以使用 clone 方法。如下
let string1 = String::from("hello");
let string2 = string1.clone();
println!("s1 = {}, s2 = {}",string1,string2);
浅拷贝
浅拷贝只发送在栈上，因此性能很高，经常见到，
let x = 4;
let y = x;
println!("x = {}, y = {}",x,y);
基本类型在编译时已知大小，拷贝值非常快，
rust 还有 copy 特征，可以用在 类似整型这样存储在栈中，如果一个类型有 copy 特征，一个旧变量在复制给新变量后，仍然可用。 基本类型，不需要分配内存资源的类型都是可以 copy 的。
所有整数类型，比如 u32
布尔类型，bool，它的值是 true 和 false
所有浮点数类型，比如 f64
字符类型，char
元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是
不可变引用 &T ，例如转移所有权中的最后一个例子，但是注意: 可变引用 &mut T 是不可以 Copy的
函数值与返回
在语义上，把值传递给函数和把值赋给变量是一致的
fn main() {
        let x = 3;
        fun1(x);
        println!("x {}",x);

        let y = String::from("hello");
        fun2(y);
        println!("y {}",y);
}
fn fun1(num : i32) {
        println!("fun1 = {}", num)
}

fn fun2(str : String) {
        println!("fun2 = {}", str)
}
// 结果
error[E0382]: borrow of moved value: `y`
  --> src/main.rs:8:18
   |
6  |     let y = String::from("hello");
   |         - move occurs because `y` has type `String`, which does not implement the `Copy` trait
7  |     fun2(y);
   |          - value moved here
8  |     println!("y {}",y);
   |                     ^ value borrowed here after move
   |
note: consider changing this parameter type in function `fun2` to borrow instead if owning the value isn't necessary
  --> src/main.rs:14:15
返回值与作用域
函数在返回值的过程中也会发生所有权转移
例子
fn main() {
    let s1 = gives_ownership();         // gives_ownership 将返回值
                                        // 移给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                        // takes_and_gives_back 中,
                                        // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
                                             // 调用它的函数

    let some_string = String::from("hello"); // some_string 进入作用域.

    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}
变量的所有权总是遵循相同模式，赋值给另一个变量时移动，当持有堆数据值的变量离开作用域时，将其通过 drop 释放，除非数据被移动到另一个变量。
虽然这样是可以的，但是在每一个函数中都获取所有权并接着返回所有权有些啰嗦。如果我们想要函数使用一个值但不获取所有权该怎么办呢？如果我们还要接着使用它的话，每次都传进去再返回来就有点烦人了，除此之外，我们也可能想返回函数体中产生的一些数据。
例子多个返回值
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = string_len(s1);
    println!("{} len {}", s2, len);
}

fn string_len(s: String) -> (String, usize){
    let len = s.len();
    (s, len)
}
// (s, s.len()) 这种形式时不可以的，String 没有 copy 
引用与借用
Rust 通过 借用(Borrowing) 这个概念来达成上述的目的，获取变量的引用，称之为借用(borrowing)。正如现实生活中，如果一个人拥有某样东西，你可以从他那里借来，当使用完毕后，也必须要物归原主。
引用：常规引用是一个指针类型，指向了对象存储的内存地址。
例子 
fn main(){
        let s1 = String::from("hello");  // 定义变量
        let len = fun1(&s1);  // 函数引用s1并不拥有 s1 , &表示引用可以看作取地址
        println!("s1 values is {}, len is {}", s1, len);
}

fn fun1(s:&String) -> usize {
        s.len()
}
不可变引用
例子
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);  // 没有改变 s1 的所有权，函数引用 s1
 
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
例子
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
// 运行结果
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:8:5
  |
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
7 | fn change(some_string: &mut String) {
  |                        ~~~~~~~~~~~

For more information about this error, try `rustc --explain E0596`.
error: could not compile `test1` due to previous error
变量默认是不可变的，引用指向的值也是不可变的
可变引用
fn main() {
    let mut s = String::from("hello");  // 定义的变量是可变的

    change(&mut s);
}

fn change(some_string: &mut String) {  // 接受的参数是可变的引用
    some_string.push_str(", world");
}
//结果
通过
可变引用同时只能存在一个
不过可变引用并不是随心所欲、想用就用的，它有一个很大的限制： 同一作用域，特定数据只能有一个可变引用：
好处：
	编译时防止数据竞争（两个或多个指针访问同一个数据，至少有一个指针用于写数据，没有任何机制来同步对数据的访问）
可以创建多个作用域，允许非同时的创建多个	可变引用（例子）
fn main() {
    let mut s = String::from("hello");
    {
        let s1 = &mut s;
    }
    let s2 = &mut s;
}
可变引用与不可变引用不可同时存在
例子
#![allow(unused)]
fn main() {
let mut s = String::from("hello");

let r1 = &s; // 没问题
let r2 = &s; // 没问题
let r3 = &mut s; // 大问题

println!("{}, {}, and {}", r1, r2, r3);
}
// 运行
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:8:10
   |
6  | let r1 = &s; // 没问题
   |          -- immutable borrow occurs here
7  | let r2 = &s; // 没问题
8  | let r3 = &mut s; // 大问题
   |          ^^^^^^ mutable borrow occurs here
9  |
10 | println!("{}, {}, and {}", r1, r2, r3);
   |                            -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `test1` due to previous error
数据竞争
两个或更多指针同时访问同一数据。
至少有一个指针被用来写入数据。
没有同步数据访问的机制。
数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复；Rust 避免了这种情况的发生，因为它甚至不会编译存在数据竞争的代码！
一如既往，可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 同时 拥有
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);

// 结果
hello and hello
hello
不可变引用 r1 和 r2 的作用域在 println! 最后一次使用之后结束，这也是创建可变引用 r3 的地方。它们的作用域没有重叠，所以代码是可以编译的。编译器可以在作用域结束之前判断不再使用的引用。
尽管这些错误有时使人沮丧，但请牢记这是 Rust 编译器在提前指出一个潜在的 bug（在编译时而不是在运行时）并精准显示问题所在。这样你就不必去跟踪为何数据并不是你想象中的那样。
悬垂引用（Dangling References）
在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。
让我们尝试创建一个悬垂引用，Rust 会通过一个编译时错误来避免：
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
引用的规则
让我们概括一下之前对引用的讨论：
在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
引用必须总是有效的。

