# 错误处理

编程有两种错误
可以被捕捉到，然后轻松处理
导致程序崩溃退出
Rust 把错误分成靓两类，可恢复和不可恢复，相当于其他语言的异常和错误。
Name	描述	范例
Recoverable	可以被捕捉，相当于其它语言的异常 Exception	Result 枚举
UnRecoverable	不可捕捉，会导致程序崩溃退出	panic 宏
可恢复错误就是可以捕捉的错误，因为可以被捕捉，所以可以矫正，让程序继续运行。
一旦捕捉到错误，程序就可以不断尝试之前那个失败操作或者选择一个备用操作。
范例：读取文件时，读取文件不存在 File not found 错误
不可恢复错误就是那些致命的会导致程序崩溃的错误，这些错误一旦发生，程序就会停止运行。
范例： 数组越界
panic!() 宏和不可恢复错误
panic!() 程序会立刻退出，并在退出时向它的调用者反馈退出原因
panic!() 格式
panic!(string_error_msg )
一般情况下，当遇到不可恢复的错误时，程序会自动调用 panic!()
也可以手动调用 panic!() 达到退出程序的目的。
示例
fn main(){
  panic!("hello");
    println!("End of main"); // 不会执行这句
}
执行结果
thread 'main' panicked at panic_test.rs:2:5:
hello
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
示例2
fn main(){
    let a = [10,20,30];
    a[10];
}
编译错误
error: this operation will panic at runtime
 --> panic_array.rs:3:5
  |
3 |     a[10];
  |     ^^^^^ index out of bounds: the length is 3 but the index is 10
  |
  = note: `#[deny(unconditional_panic)]` on by default

error: aborting due to 1 previous error
示例3
如果程序执行过程中违反了既定的业务规则，可以手动调用 panic 退出程序
fn main(){
    let n = 13;
    if n % 2 == 0 {
        println!("this is a even");
    } else {
        panic!("not an even");
    }
    println!("End of main");
}
结果
thread 'main' panicked at panic_test2.rs:6:9:
not an even
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Result 枚举和可恢复错误
一些古老的语言，比如 C 语言通过设置全局变量 error 来告诉程序发生了什么错误，然而其他语言如：JAVA 在返回类型的基础上还要通过指定可捕捉的异常来达到可恢复的目的，然而比较现代的语言，比如 Go 就是通过将错误和正常值一起返回来达到可恢复的目的。
Rust 在可恢复错误上更大胆，它使用一个 Result 枚举来封装正常返回的值和错误信息。好处就是一个变量接收正常值和错误信息，又不会污染空间。
Result<T,E> 枚举被设计用于可恢复错误。
Result 枚举定义如下：
enmu Result<T,E>{
  Ok(T),
  Err(E)
}
Result<T,E> 枚举包含了两个值：OK 和 Err。
T 和 E 则是两个范型参数：
T 用于当 Result 的值为 OK 时作为正常返回的值的数据类型。
E 用于当 Result 的值为 Err 时作为错误返回的错误的类型。
例子：Result 使用
use std::fs::File;
fn main() {
   let f = File::open("main.jpg"); //文件不存在，因此值为 Result.Err
   println!("{:?}",f);
}
结果
Err(Os { code: 2, kind: NotFound, message: "No such file or directory" })
例子2：捕捉错误信息并恢复程序运行
use std::fs::File;

fn main(){
    let f = File::open("main.jpg");

    match f {
        Ok(f) => {
            println!("file found {:?}",f);
        },
        Err(e) => {
            println!("file not found \n{:?}",e);
        }
    }

    println!("End of main");
}
运行结果
file not found 
Os { code: 2, kind: NotFound, message: "No such file or directory" }
End of main
例子3 函数返回错误
fn main(){
    let result = is_even(13);

    match result {
        Ok(d) => {
            println!("no is even: {}",d);
        },
        Err(msg) => {
            println!("Err msg is {:?}",msg);
        }
    }

    let result1 = is_even(32);

    match result1 {
        Ok(d) => {
            println!("no is even {}",d);
        },
        Err(msg) => {
            println!("Err msg is {:?}",msg);
        }
    }
}

fn is_even(no:i32) -> Result<bool, String>{
    if no%2 == 0{
        return Ok(true);
    }else{
        return Err("Not an even".to_string());
    }
}
结果
Err msg is "Not an even"
no is even true
unwrap() 函数和 expect() 函数
上面的 Result<T,E> 用 match 语句处理看起来蛮不错的样子，但写多了就会有 Go 语言漫天飞舞 if err != nil 的感觉。
有的时候我们不想处理或者让程序自己处理 Err，有的时候我们只要 Ok 值就可以了。
针对这个需求 Rust 标准库提供了两个函数 unwrap() 和 expect() 
方法	原型	说明
unwrap	unwrap(self):T	如果self 是 Ok 或者 Some 则返回包含的值。否则会调用宏 panic 退出程序
expect	expect(self,msg:&str):T	如果self 是 Ok 或者 Some 则返回包含的值。否则会输出自定义错误并退出程序
expect() 函数用于简化不希望事情失败的错误情况。而 unwrap() 函数则在返回 Ok 成功的情况下，提取返回结果。
unwrap() 和 expect() 不仅可以处理 Result<T,E> 还可以用于处理 Options<T> 枚举
示例1：正常没有异常情况
fn main(){
    let result = is_even(12).unwrap();

    println!("result is {}", result);
    println!("end of main");
}

fn is_even(no: i32) -> Result<bool, String>{
    if no % 2 == 0 {
        return Ok(true);
    }else{
        return Err("Not an even".to_string());
    }
}
结果
result is true
end of main
例子2: 异常情况
fn main(){
    let result = is_even(13).unwrap();

    println!("result is {}", result);
    println!("end of main");
}

fn is_even(no: i32) -> Result<bool, String>{
    if no % 2 == 0 {
        return Ok(true);
    }else{
        return Err("Not an even".to_string());
    }
}
结果
thread 'main' panicked at unwrap.rs:3:30:
called `Result::unwrap()` on an `Err` value: "Not an even"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
示例3: expect() 使用
use std::fs::File;
fn main(){
    let f = File::open("ptp.txt").expect("File not able to open");
    println!("End of main");
}
结果
thread 'main' panicked at expect.rs:3:35:
File not able to open: Os { code: 2, kind: NotFound, message: "No such file or directory" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
