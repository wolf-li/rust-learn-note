# Rust基础类型

## 整形
* 有符号整数（signed integers）：i8、i16、i32、i64、i128  (默认推断为 i32)  
* 无符号整数（unsigned integers）： u8、u16、u32、u64、u128   
* 由平台决定（Platform-Specific Integer Type）isize（指针宽度）、usize（指针宽度）  

## 浮点型
* 浮点数（floating point）： f32、f64  
关联常量：INFINITY(无穷大)、NEG_INFINITY(负无穷大)、NAN(非数值)、MAX(最大值)、MIN(最小值)

## char 类型
* char（字符）：单个 Unicode 字符，如 'a'，'α' 和 '∞'（每个都是 4 字节）  

## 布尔类型
* bool（布尔型）：只能是 true 或 false   

### 语法糖

* 字面量后缀 -- 类型写在数字后面
`42u8` `23i32` `3.14f64`
* 任意数字之间插入下划线方便阅读
`0xFF_i32` `1_000`

示例代码：  

{{#playground  ../code/chapter2/2_3.rs}}