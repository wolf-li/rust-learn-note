# Rust基础类型

## 整形
* 有符号整数（signed integers）：i8、i16、i32、i64、i128  (默认推断为 i32)  
* 无符号整数（unsigned integers）： u8、u16、u32、u64、u128   
* 由平台决定（Platform-Specific Integer Type）isize（指针宽度）、usize（指针宽度）  

## 浮点型
* 浮点数（floating point）： f32、f64  

## char 类型
* char（字符）：单个 Unicode 字符，如 'a'，'α' 和 '∞'（每个都是 4 字节）  

## 布尔类型
* bool（布尔型）：只能是 true 或 false  
* 单元类型（unit type）：()。其唯一可能的值就是 () 这个空元组  