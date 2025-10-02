# 复合数据类型 (Compound types)

复合类型通过基础数据类型，用来表达更加复杂的数据结构，即使用其他类型定义的类型，复合类型又被称为派生类型

## 分类

### Rust 内置的复合类型

* Struct 结构体
* Array 数组
* Tuple 元组
* Vector 动态数组

### Rust 标准库 std::collections 中的集合类型

* VecDeque
* LinkedList
* HashMap
* BTreeMap
* HashSet (不重要)
* BTreeSet (不重要)

### 按照存储方式进行划分

* 序列类型
    * 元组与数组
    * Vector
    * VecDeque
    * LinkedList
* 键值对（Map）类型 (Python 中的字典)
    * HashMap
    * BTreeMap
* 集合（Set）类型
    * HashSet
    * BTreeSet