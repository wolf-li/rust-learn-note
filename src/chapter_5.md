# 流程控制

根据条件是否为真来判断是否执行某些代码，根据条件是否为真来重复运行一段代码。
if 表达式
if 表达式允许根据条件执行不同代码分支。条件必须是布尔类型，rust 不支持将非布尔类型自动转换成为布尔类型，
fn main() {
    let x = 3;
    if x > 3 {
        println!("greater than 3");
    } else {
        println!("less than 3");
    };
}
// 结果 less than 3
if 作为表达式例子
fn main() {
    let x = 3;
    let y = if x > 3 {
        x
    } else {
        x + 3
    };
    println!("{} {}", x, y);
}
// 结果 3 6
else if 处理多重条件
fn main() {
    let x = 3;
    if x % 4 == 0 {
        println!("number is divisible by 4");
    } else if x % 3 == 0 {
        println!("number is divisible by 3");
    } else if x % 2 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    };
}
// 结果 number is divisible by 3
循环
多次执行同一块代码，rust 循环结构 loop、while、for
loop
不停的执行代码块直到明确提出停止为止。
例子 无限循环 （ctrl + c 中止程序）
fn main() {
  loop {
    println!("123");
  }
}
Rust 提供跳出循环的方法，break。 continue 关键字告诉程序跳过本次循环迭代，转到下一次迭代。
循环返回值
fn main() {
    let mut count = 0;
    let w = loop {
        if count == 10{
            break count * 9;
        }
        count += 1;
    };
    println!("{}", w);
}
// 结果 90
循环标签：如果存在循环嵌套 break 和 contine 应该应用与内层循环，可以在循环上指定循环标签，示例
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// 结果
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
while 条件循环
条件为真执行，条件为假退出。
例子
fn main() {
    let mut x = 3;
    while x != 0 {
        println!("x: {}",x);
        x -= 1;
    }
    println!("LIFTOFF");
}
// 结果
x: 3
x: 2
x: 1
LIFTOFF
遍历数组
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < a.len() {
        println!("{}",a[index]);
        index += 1;
    }
}
// 结果
10
20
30
40
50
for 循环
for 元素 in 集合 {
  // 使用元素干一些你懂我不懂的事情
}
例子
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("{}",element);
    }
}
// 结果
10
20
30
40
50
rust 标准库提供 Range 类型 ( Rev() 反转内容 )
更多 Range 提供的方法详见： https://doc.rust-lang.org/stable/std/ops/struct.Range.html
定义：start..end   
取值范围 （start <= x < end ）
(3..5) 或 std::ops::Range { start: 3, end: 5 } 或 3..5
for 循环中使用 Range 类型 
for i in 4..=7 {
    println!("{}",i);
}
// 结果
4
5
6
7

for i in -1..=4 {
    println!("{}",i);
}
// 结果
-1
0
1
2
3
4
