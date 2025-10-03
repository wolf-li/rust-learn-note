# 元组与数组

## tuple 元组

复合数据类型，可以存储多个不同类型的数据。

**Empty Tuple** `()` 为函数默认返回值。  

元组有着固定的长度。而且一旦定义，就不能再增长或缩小。
元组的下标从 0 开始。  

### 元组定义  

声明类型

`let tuple_name:(data_type1,data_type2,data_type3) = (value1,value2,value3);`  

忽略类型  
`let tuple_name = (value1,value2,value3);`  

### 获取元素

使用 `println!()` 输出所有元素，必须使用 {:?} 格式化符。

```rust
let tuple:(i32,f64,u8) = (-325,4.9,22);
println!("{:?}",tuple);
```

单独访问某个元素使用索引访问 `tuple_name.index`
```rust
let tuple:(i32,f64,u8) = (-325,4.9,22);
println!("tuple 1: {}", tuple.1);
```

### 元组没有 len() 方法

```rust
let tuple1 = (9, "lala");
println!("tuple1 len: {}", tuple1.len());
```

### 可变元组

可变元组可以修改内部元素必须同类型
```rust
let mut m_tuple = (3, "a", 5.);
println!("m_tuple first: {:?}", m_tuple);
m_tuple.1 = "fdf";
println!("m_tuple change m_tuple.1 : {:?}", m_tuple);
```

下面代码不行
```rust
# let mut m_tuple = (3, "a", 5.);
m_tuple.0 = "fdf";
println!("m_tuple change m_tuple.1 : {:?}", m_tuple);
```

## array 数组

数组：是可以存储一个固定大小的相同类型元素的顺序集合。  

### 数组特性  

* 数组的定义其实就是为分配一段 连续的相同数据类型 的内存块。

* 数组是静态的。这意味着一旦定义和初始化，则永远不可更改它的长度。

* 数组的元素有着相同的数据类型，每一个元素都独占者数据类型大小的内存块。也就是说。数组的内存大小等于数组的长度乘以数组的数据类型。

* 数组中的每一个元素都按照顺序依次存储，这个顺序号既代表着元素的存储位置，也是数组元素的唯一标识。我们把这个标识称之为 数组下标 。

* 注意，数组下标从 0 开始。

* 填充数组中的每一个元素的过程称为 数组初始化。也就是说 数组初始化 就是为数组中的每一个元素赋值。

* 可以更新或修改数组元素的值，但不能删除数组元素。如果要删除功能，你可以将它的值赋值为 0 或其它表示删除的值。

### 数组声明

1. 指定每一个元素的初始值

`let variable_name:[dataType; size] = [value1, value2, value3];`  

2. 省略数组类型

`let variable_name = [value1, value2, value3];`  

3. 默认值初始化  

`let variable_name:[dataType; size] = [default_value_for_elements; size];`  

### 获取元素

方式通过下标进行获取
`array_name[index]`  

```rust
# let arr = [0;4];
println!("arr[1]: {}", arr[1]);
```

```rust
let arr = [1..10];
println!("last arr: {}", arr[:-1]);
```

### 获取长度

通过 `len()` 方法获取
`array_name.len()`

```rust
# let arr:[u8;4] = [0;4];
println!("arr len: {}", arr.len());
```

### 遍历数组

for 方式
```rust
# let arr_for = [0,1,2,3,4];
for elem in arr_for{
    println!("{elem}");
}
```

```rust
# let arr_for = [0,1,2,3,4];
for index in 0..arr_for.len()-1{
    println!("index: {}, value: {}", index, arr_for[index]);
}
```

迭代[^1]数组 `iter()` 方式
```rust
# let arr_iter = [5,4,3,2,1,0];
for elem in arr_iter.iter(){
    println!("value is: {}", elem);
}
```

## 元组和数组的相同点

* 元组和数组都是 Compound types（复合数据类型），而 Vec 和 Map 都是 collection Types（集合数据类型）。
* 元组和数组的长度都是固定的。
* 都是可变的，添加 mut 即可。

[^1]: 迭代器让你用“函数式”写法描述“要做什么”