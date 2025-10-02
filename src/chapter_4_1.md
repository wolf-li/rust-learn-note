# 内存管理模型

## 分类

* C/C++
  * 手动写，写错就是你菜
  * new + delete | reference counting
* Python 、C# 、Java、Golang
  * 交给 GC（垃圾回收） 处理，龟派弟子突出一个猥琐
  * 安全但 stop the world[^1] 对性能伤害大
* Rust
  * The Rust Compiler 最特殊，编译器在编译时会根据一系列规则进行检查。
  * Ownership rules & semantics 、Borrow checker （引用）、 Lifetime （生命周期）

## C/C++ 内存错误大全

### 1. 内存泄漏（Memory Leaks）

描述: 已分配的内存未被释放，导致程序持续占用内存，最终可能耗尽系统资源。

```c
int *ptr = new int;
// 忘记释放内存
// delete ptr;
```

影响：进程 RSS 只增不减；长期运行服务最终 OOM 被系统杀死。

### 2. 悬空指针/野指针（Dangling Pointer / Wild Pointer）

描述: 指针指向的内存已被释放或未初始化，但指针仍被使用。

```c
int *ptr = new int;
delete ptr;
// 现在 ptr 是悬空指针,使用 ptr 进行操作
```

影响：导致未定义行为，可能修改未知内存区域，造成数据损坏、程序崩溃或安全漏洞。

### 3. 重复释放（Double free）

描述: 对同一块动态内存进行了多次释放操作。

```c
int *ptr = new int;
delete ptr;
delete ptr;
```

影响：堆元数据损坏 → 下次 malloc 时崩溃；攻击者可利用做任意写

### 4. 数组越界（Array out of bounds）

```c
int arr[4];
arr[5] = 10;
```

影响：破坏相邻变量/堆元数据；可能潜伏到后续函数才崩溃。

### 5. 野指针解引用（Dereference Wild Pointer）

```c
int *p; 
*p = 10;
```

影响：立即 Segmentation Fault 或随机写脏数据，定位困难。

### 6. 使用已经释放的内存（Use after free）

```c
int *ptr = new int;
delete ptr;
*ptr = 10;
```

### 7. 缓冲区溢出（栈/堆）（Buffer Overflow）

递归堆栈溢出

### 8. 不匹配的 new/delete 或 malloc/free

```c
int *p = new int[10];  
delete p; // 源码是 C 使用 C++ 的 delete
```

影响：C++ 未定义行为；小对象可能没事，大数组直接 heap corruption

## Rust 内存管理模型

### 所有权系统 (Ownership System)

• Rust 中的每一个值都有一个所有者（owner）  
• 值在任一时刻有且只有一个所有者  
• 当所有者（变量）离开作用域，这个值将被丢弃  

### 借用（Borrowing）

创建一个引用的行为称为 借用（borrowing）。

### 生命周期（Lifetime）

生命周期是另一类我们已经使用过的泛型。不同于确保类型有期望的行为，生命周期用于保证引用在我们需要的整个期间内都是有效的。  
Rust 中的每一个引用都有其生命周期（lifetime），也就是引用保持有效的作用域。大部分时候生命周期是隐含并可以推断的，正如大部分时候类型也是可以推断的一样。类似于当因为有多种可能类型的时候必须注明类型，也会出现引用的生命周期以一些不同方式相关联的情况，所以 Rust 需要我们使用泛型生命周期参数来注明它们的关系，这样就能确保运行时实际使用的引用绝对是有效的。

### 引用计数 (Reference Counting)

为了启用多所有权，Rust 提供了一个类型 Rc<T>，其名称为 引用计数（reference counting）的缩写。
引用计数意味着记录一个值的引用数量来知晓这个值是否仍在被使用。如果某个值有零个引用，就代表没有任何有效引用并可以被清理。可以将其想象为客厅中的电视：当最后一个人离开房间时，他关掉电视因为它不再被使用了。

[^1]: stop the world 是 GC 相关的术语，指的是在进行垃圾回收时系统暂停程序的运行。这个术语用于描述一种全局的暂停，即所有应用线程都被停止，以便垃圾回收器能够安全地进行工作。这种全局性的停止会导致一些潜在问题，特别对于一些需要低延时和高性能的应用程序。并非所有的垃圾回收算法都需要“stop the world”，现代的垃圾回收器采用一些技术来减少，全局停顿的影响，比如并发垃圾回收和增量垃圾回收。
