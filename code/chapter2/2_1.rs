# fn main(){
    // 不可变和命名
    let x = 10; // 隐式变量定义
    // x = 11;  ^^^^^^ cannot assign twice to immutable variable
    println!("x: {x}");
    let str : f64 = 7.1;  // 显式变量定义
    println!("str: {str}");

    // 声明可变
    let mut num = 4;
    println!("num: {}", num);
    num = 54;
    {
        let num = 9;
        println!("inner num: {num}");    
    }
    println!("outer num: {num}");

    let num = "hello"; // 在同一个作用域中重新声明了  num 并且改变类型，可变性，值
    println!("unmut num: {num}");

    let mut shadow = "123";
    println!("shadow: {shadow}");
    shadow = "df";
    println!("shadow: {shadow}");

    let s: String = "123".to_string();
# }